/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include <iostream>

namespace flatdata
{
namespace internal
{
const char* const INDEX_SUFFIX = "_index";

inline std::string
multivector_index_schema( const char* original_schema )
{
    return "index(" + std::string( original_schema ) + ")";
}

template < typename T >
struct ValueCreator
{
    template < typename Loader >
    std::variant< boost::optional< T >, std::string >
    operator( )( const char* resource, const char* schema, Loader&& loader )
    {
        return std::visit(
            []( auto&& data ) {
                using U = std::decay_t< decltype( data ) >;
                if constexpr ( std::is_same_v< U, std::string > )
                {
                    return data;
                }
                else if constexpr ( std::is_same_v< U, MemoryDescriptor > )
                {
                    if ( !data )
                    {
                        return boost::none;
                    }
                    return T{data.data( )};
                }
            },
            loader( resource, schema ) );
    }
};

template < typename T >
struct ValueCreator< ArrayView< T > >
{
    template < typename Loader >
    std::variant< boost::optional< ArrayView< T > >, std::string >
    operator( )( const char* resource, const char* schema, Loader&& loader )
    {
        return std::visit(
            []( auto&& data ) {
                using U = std::decay_t< decltype( data ) >;
                if constexpr ( std::is_same_v< U, std::string > )
                {
                    return data;
                }
                else if constexpr ( std::is_same_v< U, MemoryDescriptor > )
                {
                    if ( !data )
                    {
                        return boost::none;
                    }
                    return ArrayView< T >{data.data( ), data.data( ) + data.size_in_bytes( )};
                }
            },
            loader( resource, schema ) );
    }
};

template < typename IndexType, typename... Args >
struct ValueCreator< MultiArrayView< IndexType, Args... > >
{
    template < typename Loader >
    std::variant< boost::optional< MultiArrayView< IndexType, Args... > >, std::string >
    operator( )( const char* resource, const char* schema, Loader&& loader )
    {
        auto index_name = std::string( resource ) + INDEX_SUFFIX;
        auto index_result
            = loader( index_name.c_str( ), multivector_index_schema( schema ).c_str( ) );
        auto data_result = loader( resource, schema );

        return std::visit(
            [&]( auto&& index ) {
                using U = std::decay_t< decltype( index ) >;
                if constexpr ( std::is_same_v< U, std::string > )
                {
                    return index;
                }
                else if constexpr ( std::is_same_v< U, MemoryDescriptor > )
                {
                    return std::visit(
                        [&]( auto&& data ) {
                            using U = std::decay_t< decltype( data ) >;
                            if constexpr ( std::is_same_v< U, std::string > )
                            {
                                return data;
                            }
                            else if constexpr ( std::is_same_v< U, MemoryDescriptor > )
                            {
                                if ( !index && !data )
                                {
                                    return boost::none;
                                }
                                if ( !index )
                                {
                                    return index_name + " missing, but " + resource + " exists";
                                }
                                if ( !data )
                                {
                                    return std::string( resource ) + " misssing, but " + index_name
                                           + " exists";
                                }
                                ArrayView< IndexType > index_view{
                                    index.data( ), index.data( ) + index.size_in_bytes( )};
                                return MultiArrayView< IndexType, Args... >{index_view,
                                                                            data.data( )};
                            }
                        },
                        data_result );
                }
            },
            index_result );
    }
};

template <>
struct ValueCreator< MemoryDescriptor >
{
    template < typename Loader >
    std::variant< boost::optional< MemoryDescriptor >, std::string >
    operator( )( const char* resource, const char* schema, Loader&& loader )
    {
        return std::visit(
            []( auto&& data ) {
                using U = std::decay_t< decltype( data ) >;
                if constexpr ( std::is_same_v< U, std::string > )
                {
                    return data;
                }
                else if constexpr ( std::is_same_v< U, MemoryDescriptor > )
                {
                    if ( !data )
                    {
                        return boost::none;
                    }
                    return data;
                }
            },
            loader( resource, schema ) );
    }
};

class ResourceReader
{
public:
    ResourceReader( ResourceStorage* storage,
                    std::string resource_name,
                    std::string resource_schema )
        : m_storage( storage )
        , m_resource_name( std::move( resource_name ) )
        , m_resource_schema( std::move( resource_schema ) )
    {
    }

    MemoryDescriptor
    operator( )( ) const
    {
        auto data_result = m_storage->read< MemoryDescriptor >( m_resource_name.c_str( ),
                                                                m_resource_schema.c_str( ) );
        return std::visit(
            []( auto&& data ) {
                using U = std::decay_t< decltype( data ) >;
                if constexpr ( std::is_same_v< U, std::string > )
                {
                    return MemoryDescriptor( );
                }
                else if constexpr ( std::is_same_v< U, boost::optional< MemoryDescriptor > > )
                {
                    if ( !data )
                    {
                        return MemoryDescriptor( );
                    }
                    return *data;
                }
            },
            data_result );
    }

private:
    ResourceStorage* m_storage;
    std::string m_resource_name;
    std::string m_resource_schema;
};
}  // namespace internal

template < typename T >
std::variant< boost::optional< T >, std::string >
ResourceStorage::read( const char* resource_name, const char* schema )
{
    auto loader = [this]( const char* name, const char* schema ) {
        return read_and_check_schema( name, schema );
    };
    return internal::ValueCreator< T >( )( resource_name, schema, loader );
}

inline MemoryDescriptor
ResourceStorage::read_schema( const char* key )
{
    return read_resource( ( std::string( key ) + ".schema" ).c_str( ) );
}

inline bool
ResourceStorage::write_schema( const char* resource_name, const char* schema )
{
    auto schema_stream
        = create_output_stream( ( std::string( resource_name ) + ".schema" ).c_str( ) );
    *schema_stream << schema;
    schema_stream->flush( );
    return static_cast< bool >( *schema_stream );
}

inline std::variant< MemoryDescriptor, std::string >
ResourceStorage::read_and_check_schema( const char* resource_name, const char* expected_schema )
{
    auto data = read_resource( resource_name );
    auto schema = read_resource( ( std::string( resource_name ) + ".schema" ).c_str( ) );

    if ( !data && !schema )
    {
        // missing resource, might not be an error in case of optional data
        return MemoryDescriptor( );
    }

    if ( !data )
    {
        return std::string( resource_name ) + " missing, but .schema exists";
    }

    if ( !schema )
    {
        return std::string( resource_name ) + ".schema missing, but data exists";
    }

    if ( sizeof( resource_storage::size_type ) + PADDING_SIZE > data.size_in_bytes( ) )
    {
        return std::string( resource_name ) + "'s header metadata missing";
    }

    Reader< resource_storage::size_type > size_reader{data.data( )};
    auto size = size_reader;
    if ( size + sizeof( resource_storage::size_type ) + PADDING_SIZE != data.size_in_bytes( ) )
    {
        return std::string( resource_name ) + "'s size does not match metadata";
    }

    std::string stored_schema( reinterpret_cast< const char* >( schema.data( ) ),
                               schema.size_in_bytes( ) );

    if ( stored_schema != expected_schema )
    {
        auto diff = compute_diff( expected_schema, stored_schema.c_str( ) );
        return std::string( resource_name ) + "'s schema is not matching expectations:\n" + diff;
    }

    return MemoryDescriptor{data.data( ) + sizeof( resource_storage::size_type ), size};
}

template < typename T >
bool
ResourceStorage::write( const char* resource_name, const char* schema, T data )
{
    return write_to_stream( resource_name, schema, data.data( ), data.size_in_bytes( ) );
}

template < typename T >
ExternalVector< T >
ResourceStorage::create_external_vector( const char* resource_name, const char* schema )
{
    write_schema( resource_name, schema );
    auto data_stream = create_output_stream( resource_name );
    return ExternalVector< T >( ResourceHandle::create(
        std::move( data_stream ), internal::ResourceReader( this, resource_name, schema ) ) );
}

template < typename IndexType, typename... Args >
MultiVector< IndexType, Args... >
ResourceStorage::create_multi_vector( const char* resource_name, const char* schema )
{
    std::string index_name = std::string( resource_name ) + internal::INDEX_SUFFIX;
    auto index = create_external_vector< IndexType >(
        index_name.c_str( ), internal::multivector_index_schema( schema ).c_str( ) );

    write_schema( resource_name, schema );

    auto data_stream = create_output_stream( resource_name );
    return MultiVector< IndexType, Args... >(
        std::move( index ),
        ResourceHandle::create( std::move( data_stream ),
                                internal::ResourceReader( this, resource_name, schema ) ) );
}

}  // namespace flatdata

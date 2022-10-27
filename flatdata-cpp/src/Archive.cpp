/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include <flatdata/Archive.h>
#include <flatdata/internal/ArchiveUtils.h>

#include <cstring>

namespace flatdata
{
constexpr size_t TAB_WIDTH = 4;

Archive::Archive( std::shared_ptr< flatdata::ResourceStorage > storage )
    : m_storage( std::move( storage ) )
{
}

Archive::operator bool( ) const
{
    return is_open( );
}

bool
Archive::is_open( ) const
{
    return m_is_open;
}

bool
Archive::is_empty( ) const
{
    return m_is_empty;
}

bool
Archive::initialize( )
{
    if ( !m_storage )
    {
        return false;
    }

    auto signature_result = storage( ).read< flatdata::MemoryDescriptor >(
        internal::signature_name( name( ) ).c_str( ), schema( ) );

    return std::visit(
        []( auto&& data ) {
            using U = std::decay_t< decltype( data ) >;
            if constexpr ( std::is_same_v< U, std::string > )
            {
                m_errors = std::move( data );
                return false;
            }
            else if constexpr ( std::is_same_v< U, MemoryDescriptor > )
            {
                if ( !data )
                {
                    // whole archive is missing
                    return false;
                }
                m_is_open = load_contents( );
                return m_is_open;
            }
        },
        signature_result );
}

flatdata::ResourceStorage&
Archive::storage( )
{
    return *m_storage;
}

const flatdata::ResourceStorage&
Archive::storage( ) const
{
    return *m_storage;
}

std::string
Archive::describe( bool skip_resources_on_error, size_t nest_level ) const
{
    const auto newl = std::string( "\n" );
    const auto hline = std::string( 80, '=' ) + newl;
    const auto empty = std::string( "" );
    const bool is_root_node = ( nest_level == 0 );

    std::ostringstream result;

    if ( !m_errors.is_empty( ) )
    {
        if ( skip_resources_on_error )
        {
            return m_errors;
        }
        else
        {
            result << hline << m_errors << hline;
        }
    }

    if ( !m_storage )
    {
        if ( is_root_node )
        {
            result << hline << "FATAL: Resource storage not initialized. Please check archive path."
                   << newl;
        }
        else
        {
            result << "Uninitialized Archive " << name( );
        }
    }

    if ( m_storage && !m_is_open )
    {
        // Error propagated to root and storage is not initialized in respective child. No root
        // check needed.
        result << hline << "FATAL: Archive initialization failed. Unknown error." << std::endl;
    }

    if ( is_root_node )
    {
        result << hline << "Flatdata Archive: " << name( ) << std::endl
               << hline
               << "Resource                             Optional  Too Large  Loaded    Details"
               << std::endl
               << hline;
    }
    else
    {
        const std::string indent( ( nest_level - 1 ) * TAB_WIDTH, ' ' );
        result << newl + indent + std::string( "|" ) + newl + indent + std::string( "|->" )
               << " Flatdata Archive: " << name( ) << std::endl;
    }

    describe_resources( result, nest_level );

    if ( is_root_node )
    {
        result << hline;
    }

    return result.str( );
}

void
Archive::describe_impl( std::ostream& stream,
                        const char* name,
                        bool optional,
                        bool loaded,
                        const char* details,
                        bool has_nested_details,
                        bool too_large,
                        size_t nest_level )
{
    const std::string indent( nest_level * TAB_WIDTH, ' ' );
    size_t offset = indent.size( );
    stream << indent << std::left << std::setw( 37 - offset ) << std::setfill( ' ' )
           << std::string( name ).substr( 0, 30 ) << std::left << std::setw( 10 )
           << std::setfill( ' ' ) << ( optional ? "YES" : "NO" ) << std::left << std::setw( 11 )
           << std::setfill( ' ' ) << ( too_large ? "YES" : "NO" ) << std::left << std::setw( 10 )
           << std::setfill( ' ' ) << ( static_cast< bool >( loaded ) ? "YES" : "NO" ) << details
           << ( has_nested_details ? "" : "\n" );
}

}  // namespace flatdata

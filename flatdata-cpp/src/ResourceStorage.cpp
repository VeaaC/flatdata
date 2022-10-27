/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include <flatdata/ResourceStorage.h>
#include <flatdata/internal/Reader.h>
#include <flatdata/internal/ResourceHandle.h>
#include <iostream>

namespace flatdata
{
bool
ResourceStorage::write_to_stream( const char* resource_name,
                                  const char* schema,
                                  const unsigned char* data,
                                  size_t size_in_bytes )
{
    auto stream = create_output_stream( resource_name );
    resource_storage::write_to_stream( *stream, size_in_bytes );
    stream->write( reinterpret_cast< const char* >( data ), size_in_bytes );
    resource_storage::write_padding( *stream );
    stream->flush( );

    return static_cast< bool >( *stream ) && write_schema( resource_name, schema );
}

namespace
{
std::vector< std::string >
to_lines( const char* string )
{
    std::vector< std::string > result;
    if ( string )
    {
        while ( *string )
        {
            auto begin = string;
            while ( *string && *string != '\n' )
            {
                string++;
            }
            result.emplace_back( begin, string );
            if ( *string )
            {
                string++;
            }
        }
    }

    return result;
}

std::string
create_context( const std::vector< std::string >& diff )
{
    std::string result;
    std::vector< bool > result_mask( diff.size( ) );
    size_t context_lines = 3;

    int lines_to_go = -1;
    for ( size_t i = diff.size( ); i > 0; i--, lines_to_go-- )
    {
        if ( diff[ i - 1 ][ 0 ] != ' ' )
        {
            lines_to_go = context_lines;
        }

        result_mask[ i ] = lines_to_go >= 0;
    }
    lines_to_go = -1;
    for ( size_t i = 0; i < diff.size( ); i++, lines_to_go-- )
    {
        if ( diff[ i ][ 0 ] != ' ' )
        {
            lines_to_go = context_lines;
        }

        result_mask[ i ] = result_mask[ i ] || ( lines_to_go >= 0 );
    }
    bool printed_last = false;
    for ( size_t i = diff.size( ); i > 0; i-- )
    {
        if ( result_mask[ i - 1 ] )
        {
            result += diff[ i - 1 ] + '\n';
            printed_last = true;
        }
        else if ( printed_last )
        {
            result += "...\n";
            printed_last = false;
        }
    }
    return result;
}
}

std::string
ResourceStorage::compute_diff( const char* expected, const char* found )
{
    if ( !found )
    {
        return {};
    }
    std::vector< std::string > lines_expected = to_lines( expected );
    std::vector< std::string > lines_found = to_lines( found );
    size_t expected_length = lines_expected.size( ) + 1;
    size_t found_length = lines_found.size( ) + 1;

    std::vector< size_t > distances;

    distances.resize( expected_length * found_length );

    auto entry = [&]( size_t expected_pos, size_t found_pos ) -> size_t& {
        assert( expected_pos < expected_length );
        assert( found_pos < found_length );
        return distances[ expected_pos + found_pos * expected_length ];
    };

    for ( size_t i = 0; i < expected_length; i++ )
    {
        entry( i, 0 ) = i;
    }
    for ( size_t i = 0; i < found_length; i++ )
    {
        entry( 0, i ) = i;
    }

    // fill in distance table
    for ( size_t found_pos = 0; found_pos < lines_found.size( ); found_pos++ )
    {
        for ( size_t expected_pos = 0; expected_pos < lines_expected.size( ); expected_pos++ )
        {
            size_t min_cost = std::min( entry( expected_pos, found_pos + 1 ) + 1,
                                        entry( expected_pos + 1, found_pos ) + 1 );

            if ( lines_expected[ expected_pos ] == lines_found[ found_pos ] )
            {
                min_cost = std::min( min_cost, entry( expected_pos, found_pos ) );
            }

            entry( expected_pos + 1, found_pos + 1 ) = min_cost;
        }
    }

    std::vector< std::string > diff;
    size_t found_pos = lines_found.size( );
    size_t expected_pos = lines_expected.size( );
    while ( found_pos != 0 || expected_pos != 0 )
    {
        size_t next_found_pos = found_pos;
        size_t next_expected_pos = expected_pos;
        auto check = [&]( size_t new_expected_pos, size_t new_found_pos, size_t transition_cost ) {
            size_t cost = entry( new_expected_pos, new_found_pos );
            if ( cost + transition_cost == entry( expected_pos, found_pos ) )
            {
                next_found_pos = new_found_pos;
                next_expected_pos = new_expected_pos;
            }
        };
        if ( found_pos != 0 )
        {
            check( expected_pos, found_pos - 1, 1 );
        }
        if ( expected_pos != 0 )
        {
            check( expected_pos - 1, found_pos, 1 );
        }
        if ( expected_pos != 0 && found_pos != 0
             && lines_expected[ expected_pos - 1 ] == lines_found[ found_pos - 1 ] )
        {
            check( expected_pos - 1, found_pos - 1, 0 );
        }
        assert( expected_pos != next_expected_pos || found_pos != next_found_pos );
        if ( next_found_pos == found_pos )
        {
            diff.emplace_back( std::string( "-" ) + "\"" + lines_expected[ next_expected_pos ]
                               + "\"" );
        }
        else if ( next_expected_pos == expected_pos )
        {
            diff.emplace_back( std::string( "+" ) + "\"" + lines_found[ next_found_pos ] + "\"" );
        }
        else
        {
            diff.emplace_back( std::string( " " ) + "\"" + lines_expected[ next_expected_pos ]
                               + "\"" );
        }
        found_pos = next_found_pos;
        expected_pos = next_expected_pos;
    }

    return create_context( diff );
}

}  // namespace flatdata

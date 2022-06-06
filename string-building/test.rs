#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use const_format::{concatcp, str_get};
const NAME: &str = "Christof";
struct MeType {
    name: &'static str,
}
const fn get_me() -> MeType {
    MeType { name: "Christof" }
}
const fn hello_me() -> &'static str {
    ({
        use ::const_format::__cf_osRcTFl4A;
        ({
            #[allow(unused_mut, non_snake_case)]
            const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                &[
                    __cf_osRcTFl4A::pmr::PConvWrapper("Hello ").to_pargument_display(fmt),
                    __cf_osRcTFl4A::pmr::PConvWrapper(get_me().name).to_pargument_display(fmt),
                ]
            };
            {
                const ARR_LEN: usize =
                    ::const_format::pmr::PArgument::calc_len(CONCATP_NHPMWYD3NJA);
                const CONCAT_ARR: &::const_format::pmr::LenAndArray<[u8; ARR_LEN]> = {
                    use ::const_format::{__write_pvariant, pmr::PVariant};
                    let mut out = ::const_format::pmr::LenAndArray {
                        len: 0,
                        array: [0u8; ARR_LEN],
                    };
                    let input = CONCATP_NHPMWYD3NJA;
                    {
                        let ::const_format::pmr::Range {
                            start: mut outer_i,
                            end,
                        } = 0..input.len();
                        while outer_i < end {
                            {
                                let current = &input[outer_i];
                                match current.elem {
                                    PVariant::Str(s) => {
                                        let str = s.as_bytes();
                                        let is_display = current.fmt.is_display();
                                        let mut i = 0;
                                        if is_display {
                                            while i < str.len() {
                                                out.array[out.len] = str[i];
                                                out.len += 1;
                                                i += 1;
                                            }
                                        } else {
                                            out.array[out.len] = b'"';
                                            out.len += 1;
                                            while i < str.len() {
                                                use ::const_format::pmr::{
                                                    hex_as_ascii, ForEscaping, FOR_ESCAPING,
                                                };
                                                let c = str[i];
                                                let mut written_c = c;
                                                if c < 128 {
                                                    let shifted = 1 << c;
                                                    if (FOR_ESCAPING.is_escaped & shifted) != 0 {
                                                        out.array[out.len] = b'\\';
                                                        out.len += 1;
                                                        if (FOR_ESCAPING.is_backslash_escaped
                                                            & shifted)
                                                            == 0
                                                        {
                                                            out.array[out.len] = b'x';
                                                            out.array[out.len + 1] =
                                                                hex_as_ascii(c >> 4);
                                                            out.len += 2;
                                                            written_c = hex_as_ascii(c & 0b1111);
                                                        } else {
                                                            written_c =
                                                                ForEscaping::get_backslash_escape(
                                                                    c,
                                                                );
                                                        };
                                                    }
                                                }
                                                out.array[out.len] = written_c;
                                                out.len += 1;
                                                i += 1;
                                            }
                                            out.array[out.len] = b'"';
                                            out.len += 1;
                                        }
                                    }
                                    PVariant::Int(int) => {
                                        let wrapper = ::const_format::pmr::PWrapper(int);
                                        let debug_display;
                                        let bin;
                                        let hex;
                                        let sa : & :: const_format :: pmr :: StartAndArray < [_] > = match current . fmt { :: const_format :: pmr :: Formatting :: Display => { debug_display = wrapper . to_start_array_display () ; & debug_display } :: const_format :: pmr :: Formatting :: Debug => match current . fmt_flags . num_fmt () { :: const_format :: pmr :: NumberFormatting :: Decimal => { debug_display = wrapper . to_start_array_debug () ; & debug_display } :: const_format :: pmr :: NumberFormatting :: Binary => { bin = wrapper . to_start_array_binary (current . fmt_flags) ; & bin } :: const_format :: pmr :: NumberFormatting :: Hexadecimal => { hex = wrapper . to_start_array_hexadecimal (current . fmt_flags) ; & hex } } , } ;
                                        let mut start = sa.start;
                                        while start < sa.array.len() {
                                            out.array[out.len] = sa.array[start];
                                            out.len += 1;
                                            start += 1;
                                        }
                                    }
                                    PVariant::Char(c) => {
                                        let encoded = c.encoded();
                                        let len = c.len();
                                        let mut start = 0;
                                        while start < len {
                                            out.array[out.len] = encoded[start];
                                            out.len += 1;
                                            start += 1;
                                        }
                                    }
                                }
                            }
                            outer_i += 1;
                        }
                    }
                    &{ out }
                };
                #[allow(clippy::transmute_ptr_to_ptr)]
                const CONCAT_STR: &str = unsafe {
                    let slice = ::const_format::pmr::transmute::<
                        &[u8; ARR_LEN],
                        &[u8; CONCAT_ARR.len],
                    >(&CONCAT_ARR.array);
                    {
                        let bytes: &'static [::const_format::pmr::u8] = slice;
                        let string: &'static ::const_format::pmr::str = {
                            ::const_format::__hidden_utils::PtrToRef {
                                ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                            }
                            .reff
                        };
                        string
                    }
                };
                CONCAT_STR
            }
        })
    } as &'static ::const_format::pmr::str)
}
const ME_NAME: &'static str = {
    let me = get_me();
    me.name
};
const fn get_string_length(input: &str) -> usize {
    input.len()
}
const NAME_LENGTH: usize = get_string_length(NAME);
struct TreeElement<'a>
where
    T: Fn,
{
    pub handle: &'a T,
    pub path: &'static str,
}
fn test(_me: i32, hey: u8) -> u8 {
    2u8
}
fn test2(_me: i32, hey: i32) -> u128 {
    2u128
}
const fn get_duplicates(inputs: &[&str]) -> usize {
    let mut dupe_count = 0;
    let mut count = 0;
    dupe_count
}
fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["str NAME: ", "\n"],
            &[::core::fmt::ArgumentV1::new_display(&NAME)],
        ));
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["String NAME: ", "!\n"],
            &[::core::fmt::ArgumentV1::new_display(&ME_NAME)],
        ));
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["get_me().name: ", "!\n"],
            &[::core::fmt::ArgumentV1::new_display(&get_me().name)],
        ));
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["hello_me(): ", "!\n"],
            &[::core::fmt::ArgumentV1::new_display(&hello_me())],
        ));
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["NAME_LENGTH: ", "!\n"],
            &[::core::fmt::ArgumentV1::new_display(&NAME_LENGTH)],
        ));
    };
    const element: TreeElement<i32, u8, u8> = TreeElement {
        handle: &test,
        path: "/",
    };
    const element2: TreeElement<i32, i32, u128> = TreeElement {
        handle: &test2,
        path: "/test",
    };
    const handle: &dyn Fn(i32, u8) -> u8 = element.handle;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["Value: ", "\n"],
            &[::core::fmt::ArgumentV1::new_display(&handle(4i32, 4u8))],
        ));
    };
    const handle2: &dyn Fn(i32, i32) -> u128 = element2.handle;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["Value: ", "\n"],
            &[::core::fmt::ArgumentV1::new_display(&handle2(4i32, 4i32))],
        ));
    };
}

use std::num::ParseFloatError;

use super::normal_prop;
use crate::prop::ALIGN;
use gen_utils::common::token_stream_to_tree;
use proc_macro2::TokenTree;
use quote::quote;

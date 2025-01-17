// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::*;

pub(crate) fn new() -> Item {
    let docs = docs!();
    let name = "serial".into();
    let items = vec![
        item! {
            /// Reads from USB serial into a buffer.
            fn read "usr" {
                /// Address of the buffer.
                ptr: *mut u8,

                /// Length of the buffer in bytes.
                len: usize,
            } -> {
                /// Number of bytes read (or negative value for errors).
                ///
                /// This function does not block and may return zero.
                len: isize,
            }
        },
        item! {
            /// Writes to USB serial from a buffer.
            fn write "usw" {
                /// Address of the buffer.
                ptr: *const u8,

                /// Length of the buffer in bytes.
                len: usize,
            } -> {
                /// Number of bytes written (or negative value for errors).
                ///
                /// This function does not block and may return zero.
                len: isize,
            }
        },
        item! {
            /// USB serial events.
            enum Event {
                /// Ready for read.
                Read = 0,
                /// Ready for write.
                Write = 1,
            }
        },
        item! {
            /// Registers a callback when USB serial is ready.
            ///
            /// It is possible that the callback is spuriously called.
            fn register "use" {
                event: usize,
                handler_func: fn { data: *const u8 },
                handler_data: *const u8,
            } -> {}
        },
        item! {
            /// Unregisters a callback.
            fn unregister "usd" {
                event: usize,
            } -> {}
        },
        item! {
            /// Flushs the USB serial.
            fn flush "usf" {} -> {
                /// Zero on success, -1 on error.
                res: isize,
            }
        },
    ];
    Item::Mod(Mod { docs, name, items })
}

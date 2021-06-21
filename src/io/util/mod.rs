#![allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411

cfg_io_util! {
    mod async_write_ext;
    pub use async_write_ext::AsyncWriteExt;

    mod flush;
    mod shutdown;
    mod write;
    mod write_vectored;
    mod write_all;
    mod write_buf;
    mod write_all_buf;
    mod write_int;
}

cfg_not_io_util! {
    cfg_process! {
        mod vec_with_initialized;
        mod read_to_end;
        // Used by process
        pub(crate) use read_to_end::read_to_end;
    }
}

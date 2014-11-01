extern crate libc;

#[repr(C)]
pub struct ogg_iovec_t;
#[repr(C)]
pub struct oggpack_buffer;
#[repr(C)]
pub struct ogg_page;
#[repr(C)]
pub struct ogg_stream_state;
#[repr(C)]
pub struct ogg_packet;
#[repr(C)]
pub struct ogg_sync_state;

extern {
    pub fn oggpack_writeinit(b: *mut oggpack_buffer);
    pub fn oggpack_writecheck(b: *mut oggpack_buffer) -> libc::c_int;
    pub fn oggpack_writetrunc(b: *mut oggpack_buffer, bits: libc::c_long);
    pub fn oggpack_writealign(b: *mut oggpack_buffer);
    pub fn oggpack_writecopy(b: *mut oggpack_buffer, source: *mut libc::c_void, bits: libc::c_long);
    pub fn oggpack_reset(b: *mut oggpack_buffer);
    pub fn oggpack_writeclear(b: *mut oggpack_buffer);
    pub fn oggpack_readinit(b: *mut oggpack_buffer, buf: *mut libc::c_uchar, bytes: libc::c_int)
        -> libc::c_int;
    pub fn oggpack_write(b: *mut oggpack_buffer, value: libc::c_ulong, bits: libc::c_int)
        -> libc::c_int;
    pub fn oggpack_look(b: *mut oggpack_buffer, bits: libc::c_int) -> libc::c_long;
    pub fn oggpack_look1(b: *mut oggpack_buffer) -> libc::c_long;
    pub fn oggpack_adv(b: *mut oggpack_buffer, bits: libc::c_int) -> libc::c_int;
    pub fn oggpack_adv1(b: *mut oggpack_buffer);
    pub fn oggpack_read(b: *mut oggpack_buffer, bits: libc::c_int) -> libc::c_long;
    pub fn oggpack_read1(b: *mut oggpack_buffer) -> libc::c_long;
    pub fn oggpack_bytes(b: *mut oggpack_buffer) -> libc::c_long;
    pub fn oggpack_bits(b: *mut oggpack_buffer) -> libc::c_long;
    pub fn oggpack_get_buffer(b: *mut oggpack_buffer) -> *const libc::c_uchar;

    pub fn oggpackB_writeinit(b: *mut oggpack_buffer);
    pub fn oggpackB_writecheck(b: *mut oggpack_buffer) -> libc::c_int;
    pub fn oggpackB_writetrunc(b: *mut oggpack_buffer, bits: libc::c_long);
    pub fn oggpackB_writealign(b: *mut oggpack_buffer);
    pub fn oggpackB_writecopy(b: *mut oggpack_buffer, source: *mut libc::c_void,
        bits: libc::c_long);
    pub fn oggpackB_reset(b: *mut oggpack_buffer);
    pub fn oggpackB_writeclear(b: *mut oggpack_buffer);
    pub fn oggpackB_readinit(b: *mut oggpack_buffer, buf: *mut libc::c_uchar, bytes: libc::c_int)
        -> libc::c_int;
    pub fn oggpackB_write(b: *mut oggpack_buffer, value: libc::c_ulong, bits: libc::c_int)
        -> libc::c_int;
    pub fn oggpackB_look(b: *mut oggpack_buffer,bits: libc::c_int) -> libc::c_long;
    pub fn oggpackB_look1(b: *mut oggpack_buffer) -> libc::c_long;
    pub fn oggpackB_adv(b: *mut oggpack_buffer, bits: libc::c_int) -> libc::c_int;
    pub fn oggpackB_adv1(b: *mut oggpack_buffer);
    pub fn oggpackB_read(b: *mut oggpack_buffer, bits: libc::c_int) -> libc::c_long;
    pub fn oggpackB_read1(b: *mut oggpack_buffer) -> libc::c_long;
    pub fn oggpackB_bytes(b: *mut oggpack_buffer) -> libc::c_long;
    pub fn oggpackB_bits(b: *mut oggpack_buffer) -> libc::c_long;
    pub fn oggpackB_get_buffer(b: *mut oggpack_buffer) -> *const libc::c_uchar;

    pub fn ogg_stream_packetin(os: *mut ogg_stream_state, op: *mut ogg_packet) -> libc::c_int;
    pub fn ogg_stream_iovecin(os: *mut ogg_stream_state, iov: *mut ogg_iovec_t, count: libc::c_int,
        e_o_s: libc::c_long, granulepos: libc::int64_t) -> libc::c_int;
    pub fn ogg_stream_pageout(os: *mut ogg_stream_state, og: *mut ogg_page) -> libc::c_int;
    pub fn ogg_stream_pageout_fill(os: *mut ogg_stream_state, og: *mut ogg_page,
        nfill: libc::c_int) -> libc::c_int;
    pub fn ogg_stream_flush(os: *mut ogg_stream_state, og: *mut ogg_page) -> libc::c_int;
    pub fn ogg_stream_flush_fill(os: *mut ogg_stream_state, og: *mut ogg_page,
        nfill: libc::c_int) -> libc::c_int;

    pub fn ogg_sync_init(oy: *mut ogg_sync_state) -> libc::c_int;
    pub fn ogg_sync_clear(oy: *mut ogg_sync_state) -> libc::c_int;
    pub fn ogg_sync_reset(oy: *mut ogg_sync_state) -> libc::c_int;
    pub fn ogg_sync_destroy(oy: *mut ogg_sync_state) -> libc::c_int;
    pub fn ogg_sync_check(oy: *mut ogg_sync_state) -> libc::c_int;

    pub fn ogg_sync_buffer(oy: *mut ogg_sync_state, size: libc::c_long) -> *const libc::c_char;
    pub fn ogg_sync_wrote(oy: *mut ogg_sync_state, bytes: libc::c_long) -> libc::c_int;
    pub fn ogg_sync_pageseek(oy: *mut ogg_sync_state,og: *mut ogg_page) -> libc::c_long;
    pub fn ogg_sync_pageout(oy: *mut ogg_sync_state, og: *mut ogg_page) -> libc::c_int;
    pub fn ogg_stream_pagein(os: *mut ogg_stream_state, og: *mut ogg_page) -> libc::c_int;
    pub fn ogg_stream_packetout(os: *mut ogg_stream_state,op: *mut ogg_packet) -> libc::c_int;
    pub fn ogg_stream_packetpeek(os: *mut ogg_stream_state,op: *mut ogg_packet) -> libc::c_int;

    pub fn ogg_stream_init(os: *mut ogg_stream_state, serialno: libc::c_int) -> libc::c_int;
    pub fn ogg_stream_clear(os: *mut ogg_stream_state) -> libc::c_int;
    pub fn ogg_stream_reset(os: *mut ogg_stream_state) -> libc::c_int;
    pub fn ogg_stream_reset_serialno(os: *mut ogg_stream_state, serialno: libc::c_int)
        -> libc::c_int;
    pub fn ogg_stream_destroy(os: *mut ogg_stream_state) -> libc::c_int;
    pub fn ogg_stream_check(os: *mut ogg_stream_state) -> libc::c_int;
    pub fn ogg_stream_eos(os: *mut ogg_stream_state) -> libc::c_int;

    pub fn ogg_page_checksum_set(og: *mut ogg_page);

    pub fn ogg_page_version(og: *const ogg_page) -> libc::c_int;
    pub fn ogg_page_continued(og: *const ogg_page) -> libc::c_int;
    pub fn ogg_page_bos(og: *const ogg_page) -> libc::c_int;
    pub fn ogg_page_eos(og: *const ogg_page) -> libc::c_int;
    pub fn ogg_page_granulepos(og: *const ogg_page) -> libc::int64_t;
    pub fn ogg_page_serialno(og: *const ogg_page) -> libc::c_int;
    pub fn ogg_page_pageno(og: *const ogg_page) -> libc::c_long;
    pub fn ogg_page_packets(og: *const ogg_page) -> libc::c_int;

    pub fn ogg_packet_clear(op: *mut ogg_packet);

}

pub use crate::ffopen as fits_open_file;

pub use crate::ffbinr as fits_parse_binrang;
pub use crate::ffbins as fits_parse_binspec;
pub use crate::ffexist as fits_file_exists;
pub use crate::ffextn as fits_parse_extnum;
pub use crate::ffexts as fits_parse_extspec;
pub use crate::ffifile as fits_parse_input_filename;
pub use crate::ffiurl as fits_parse_input_url;
pub use crate::ffomem as fits_open_memfile;
// pub use crate::ffourl as fits_parse_output_url;
pub use crate::ffrtnm as fits_parse_rootname;
pub use crate::ffrwrg as fits_parse_range;
pub use crate::ffrwrgll as fits_parse_rangell;

pub use crate::ffclos as fits_close_file;
pub use crate::ffdelt as fits_delete_file;
pub use crate::ffdkinit as fits_create_diskfil;
pub use crate::ffdkopn as fits_open_diskfile;
pub use crate::ffdopn as fits_open_data;
pub use crate::ffeopn as fits_open_extlist;
pub use crate::ffflmd as fits_file_mode;
pub use crate::ffflnm as fits_file_name;
pub use crate::ffflsh as fits_flush_buffer;
pub use crate::ffflus as fits_flush_file;
pub use crate::ffimem as fits_create_memfil;
pub use crate::ffinit as fits_create_file;
pub use crate::ffiopn as fits_open_image;
pub use crate::ffreopen as fits_reopen_file;
pub use crate::fftopn as fits_open_table;
pub use crate::fftplt as fits_create_templat;
pub use crate::ffurlt as fits_url_type;

pub use crate::ffasfm as fits_ascii_tform;
pub use crate::ffbnfm as fits_binary_tform;
pub use crate::ffbnfmll as fits_binary_tformll;
pub use crate::ffcmps as fits_compare_str;
pub use crate::ffcmrk as fits_clear_errmark;
pub use crate::ffcmsg as fits_clear_errmsg;
pub use crate::ffdtyp as fits_get_keytype;
pub use crate::ffgabc as fits_get_tbcol;
pub use crate::ffgcdw as fits_get_col_display_width;
pub use crate::ffgerr as fits_get_errstatus;
pub use crate::ffgkcl as fits_get_keyclass;
pub use crate::ffgknm as fits_get_keyname;
pub use crate::ffgmsg as fits_read_errmsg;
pub use crate::ffgrsz as fits_get_rowsize;
pub use crate::ffgthd as fits_parse_templat;
pub use crate::ffinttyp as fits_get_inttype;
pub use crate::ffkeyn as fits_make_keyn;
pub use crate::ffmkky as fits_make_key;
pub use crate::ffnchk as fits_null_check;
pub use crate::ffnkey as fits_make_nkey;
pub use crate::ffpmrk as fits_write_errmark;
pub use crate::ffpmsg as fits_write_errmsg;
pub use crate::ffpsvc as fits_parse_value;
pub use crate::ffrprt as fits_report_error;
pub use crate::fftkey as fits_test_keyword;
pub use crate::fftrec as fits_test_record;
pub use crate::ffupch as fits_uppercase;
pub use crate::ffvers as fits_get_version;

pub use crate::ffcpky as fits_copy_key;
pub use crate::ffdt2s as fits_date2str;
pub use crate::ffgsdt as fits_get_system_date;
pub use crate::ffgstm as fits_get_system_time;
pub use crate::ffpcom as fits_write_comment;
pub use crate::ffpdat as fits_write_date;
pub use crate::ffphbn as fits_write_btblhdr;
pub use crate::ffphext as fits_write_exthdr;
pub use crate::ffphis as fits_write_history;
pub use crate::ffphpr as fits_write_grphdr;
pub use crate::ffphprll as fits_write_grphdrll;
pub use crate::ffphps as fits_write_imghdr;
pub use crate::ffphpsll as fits_write_imghdrll;
pub use crate::ffphtb as fits_write_atblhdr;
pub use crate::ffpkfc as fits_write_key_fixcmp;
pub use crate::ffpkfm as fits_write_key_fixdblcm;
pub use crate::ffpkls as fits_write_key_longstr;
pub use crate::ffpknd as fits_write_keys_dbl;
pub use crate::ffpkne as fits_write_keys_flt;
pub use crate::ffpknf as fits_write_keys_fixflt;
pub use crate::ffpkng as fits_write_keys_fixdbl;
pub use crate::ffpknj as fits_write_keys_lng;
pub use crate::ffpknl as fits_write_keys_log;
pub use crate::ffpkns as fits_write_keys_str;
pub use crate::ffpktp as fits_write_key_templat;
pub use crate::ffpky as fits_write_key;
pub use crate::ffpkyc as fits_write_key_cmp;
pub use crate::ffpkyd as fits_write_key_dbl;
pub use crate::ffpkye as fits_write_key_flt;
pub use crate::ffpkyf as fits_write_key_fixflt;
pub use crate::ffpkyg as fits_write_key_fixdbl;
pub use crate::ffpkyj as fits_write_key_lng;
pub use crate::ffpkyl as fits_write_key_log;
pub use crate::ffpkym as fits_write_key_dblcmp;
pub use crate::ffpkys as fits_write_key_str;
pub use crate::ffpkyt as fits_write_key_triple;
pub use crate::ffpkyu as fits_write_key_null;
pub use crate::ffpkyuj as fits_write_key_ulng;
pub use crate::ffplsw as fits_write_key_longwar;
pub use crate::ffprec as fits_write_record;
pub use crate::ffptdm as fits_write_tdim;
pub use crate::ffptdmll as fits_write_tdimll;
pub use crate::ffpunt as fits_write_key_unit;
pub use crate::ffs2dt as fits_str2date;
pub use crate::ffs2tm as fits_str2time;
pub use crate::fftm2s as fits_time2str;

pub use crate::ffghps as fits_get_hdrpos;
pub use crate::ffghsp as fits_get_hdrspace;
pub use crate::ffgnxk as fits_find_nextkey;
pub use crate::ffmaky as fits_movabs_key;
pub use crate::ffmrky as fits_movrel_key;

pub use crate::ffcnvthdr2str as fits_convert_hdr2str;
pub use crate::ffdtdm as fits_decode_tdim;
pub use crate::ffdtdmll as fits_decode_tdimll;
pub use crate::fffree as fits_free_memory;
pub use crate::ffgcrd as fits_read_card;
pub use crate::ffghbn as fits_read_btblhdr;
pub use crate::ffghbnll as fits_read_btblhdrll;
pub use crate::ffghpr as fits_read_imghdr;
pub use crate::ffghprll as fits_read_imghdrll;
pub use crate::ffghtb as fits_read_atblhdr;
pub use crate::ffghtbll as fits_read_atblhdrll;
pub use crate::ffgkey as fits_read_keyword;
pub use crate::ffgkls as fits_read_key_longstr;
pub use crate::ffgknd as fits_read_keys_dbl;
pub use crate::ffgkne as fits_read_keys_flt;
pub use crate::ffgknj as fits_read_keys_lng;
pub use crate::ffgknjj as fits_read_keys_lnglng;
pub use crate::ffgknl as fits_read_keys_log;
pub use crate::ffgkns as fits_read_keys_str;
pub use crate::ffgksl as fits_get_key_strlen;
pub use crate::ffgky as fits_read_key;
pub use crate::ffgkyc as fits_read_key_cmp;
pub use crate::ffgkyd as fits_read_key_dbl;
pub use crate::ffgkye as fits_read_key_flt;
pub use crate::ffgkyj as fits_read_key_lng;
pub use crate::ffgkyjj as fits_read_key_lnglng;
pub use crate::ffgkyl as fits_read_key_log;
pub use crate::ffgkym as fits_read_key_dblcmp;
pub use crate::ffgkyn as fits_read_keyn;
pub use crate::ffgkys as fits_read_key_str;
pub use crate::ffgkyt as fits_read_key_triple;
pub use crate::ffgkyujj as fits_read_key_ulnglng;
pub use crate::ffgrec as fits_read_record;
pub use crate::ffgsky as fits_read_string_key;
pub use crate::ffgstr as fits_read_str;
pub use crate::ffgtdm as fits_read_tdim;
pub use crate::ffgtdmll as fits_read_tdimll;
pub use crate::ffgunt as fits_read_key_unit;
pub use crate::ffhdr2str as fits_hdr2str;

pub use crate::ffucrd as fits_update_card;
pub use crate::ffukfc as fits_update_key_fixcm;
pub use crate::ffukfm as fits_update_key_fixdblcm;
pub use crate::ffukls as fits_update_key_longstr;
pub use crate::ffuky as fits_update_key;
pub use crate::ffukyc as fits_update_key_cmp;
pub use crate::ffukyd as fits_update_key_dbl;
pub use crate::ffukye as fits_update_key_flt;
pub use crate::ffukyf as fits_update_key_fixfl;
pub use crate::ffukyg as fits_update_key_fixdb;
pub use crate::ffukyj as fits_update_key_lng;
pub use crate::ffukyl as fits_update_key_log;
pub use crate::ffukym as fits_update_key_dblcm;
pub use crate::ffukys as fits_update_key_str;
pub use crate::ffukyu as fits_update_key_null;

pub use crate::ffmcom as fits_modify_comment;
pub use crate::ffmcrd as fits_modify_card;
pub use crate::ffmkfc as fits_modify_key_fixcm;
pub use crate::ffmkfm as fits_modify_key_fixdblcm;
pub use crate::ffmkls as fits_modify_key_longstr;
pub use crate::ffmkyc as fits_modify_key_cmp;
pub use crate::ffmkyd as fits_modify_key_dbl;
pub use crate::ffmkye as fits_modify_key_flt;
pub use crate::ffmkyf as fits_modify_key_fixfl;
pub use crate::ffmkyg as fits_modify_key_fixdb;
pub use crate::ffmkyj as fits_modify_key_lng;
pub use crate::ffmkyl as fits_modify_key_log;
pub use crate::ffmkym as fits_modify_key_dblcm;
pub use crate::ffmkys as fits_modify_key_str;
pub use crate::ffmkyu as fits_modify_key_null;
pub use crate::ffmnam as fits_modify_name;
pub use crate::ffmrec as fits_modify_record;

pub use crate::ffikey as fits_insert_card;
pub use crate::ffikfc as fits_insert_key_fixcm;
pub use crate::ffikfm as fits_insert_key_fixdblcm;
pub use crate::ffikls as fits_insert_key_longstr;
pub use crate::ffikyc as fits_insert_key_cmp;
pub use crate::ffikyd as fits_insert_key_dbl;
pub use crate::ffikye as fits_insert_key_flt;
pub use crate::ffikyf as fits_insert_key_fixfl;
pub use crate::ffikyg as fits_insert_key_fixdb;
pub use crate::ffikyj as fits_insert_key_lng;
pub use crate::ffikyl as fits_insert_key_log;
pub use crate::ffikym as fits_insert_key_dblcm;
pub use crate::ffikys as fits_insert_key_str;
pub use crate::ffikyu as fits_insert_key_null;
pub use crate::ffirec as fits_insert_record;

pub use crate::ffdkey as fits_delete_key;
pub use crate::ffdrec as fits_delete_record;
pub use crate::ffdstr as fits_delete_str;
pub use crate::ffghad as fits_get_hduaddr;
pub use crate::ffghadll as fits_get_hduaddrll;
pub use crate::ffghdn as fits_get_hdu_num;
pub use crate::ffghdt as fits_get_hdu_type;
pub use crate::ffghof as fits_get_hduoff;

pub use crate::ffgipr as fits_get_img_param;
pub use crate::ffgiprll as fits_get_img_paramll;

pub use crate::ffgidm as fits_get_img_dim;
pub use crate::ffgidt as fits_get_img_type;
pub use crate::ffgiet as fits_get_img_equivtype;
pub use crate::ffgisz as fits_get_img_size;
pub use crate::ffgiszll as fits_get_img_sizell;

pub use crate::ffcrhd as fits_create_hdu;
pub use crate::ffcrim as fits_create_img;
pub use crate::ffcrimll as fits_create_imgll;
pub use crate::ffcrtb as fits_create_tbl;
pub use crate::ffibin as fits_insert_btbl;
pub use crate::ffiimg as fits_insert_img;
pub use crate::ffiimgll as fits_insert_imgll;
pub use crate::ffitab as fits_insert_atbl;
pub use crate::ffmahd as fits_movabs_hdu;
pub use crate::ffmnhd as fits_movnam_hdu;
pub use crate::ffmrhd as fits_movrel_hdu;
pub use crate::ffrsim as fits_resize_img;
pub use crate::ffrsimll as fits_resize_imgll;
pub use crate::ffthdu as fits_get_num_hdus;

pub use crate::ffcopy as fits_copy_hdu;
pub use crate::ffcpdt as fits_copy_data;
pub use crate::ffcpfl as fits_copy_file;
pub use crate::ffcphd as fits_copy_header;
pub use crate::ffcpht as fits_copy_hdutab;
pub use crate::ffdhdu as fits_delete_hdu;
pub use crate::ffwrhdu as fits_write_hdu;

pub use crate::ffhdef as fits_set_hdrsize;
pub use crate::ffpthp as fits_write_theap;
pub use crate::ffrdef as fits_set_hdustruc;

pub use crate::ffdsum as fits_decode_chksum;
pub use crate::ffesum as fits_encode_chksum;
pub use crate::ffgcks as fits_get_chksum;
pub use crate::ffpcks as fits_write_chksum;
pub use crate::ffupck as fits_update_chksum;
pub use crate::ffvcks as fits_verify_chksum;

pub use crate::ffpnul as fits_set_imgnull;
pub use crate::ffpscl as fits_set_bscale;
pub use crate::ffsnul as fits_set_atblnull;
pub use crate::fftnul as fits_set_btblnull;
pub use crate::fftscl as fits_set_tscale;

pub use crate::ffeqty as fits_get_eqcoltype;
pub use crate::ffeqtyll as fits_get_eqcoltypel;
pub use crate::ffgacl as fits_get_acolparms;
pub use crate::ffgbcl as fits_get_bcolparms;
pub use crate::ffgbclll as fits_get_bcolparmsll;
pub use crate::ffgcnn as fits_get_colname;
pub use crate::ffgcno as fits_get_colnum;
pub use crate::ffgncl as fits_get_num_cols;
pub use crate::ffgnrw as fits_get_num_rows;
pub use crate::ffgnrwll as fits_get_num_rowsll;
pub use crate::ffgtcl as fits_get_coltype;
pub use crate::ffgtclll as fits_get_coltypell;

pub use crate::ffiter as fits_iterate_data;

pub use crate::ffggpb as fits_read_grppar_byt;
pub use crate::ffggpd as fits_read_grppar_dbl;
pub use crate::ffggpe as fits_read_grppar_flt;
pub use crate::ffggpi as fits_read_grppar_sht;
pub use crate::ffggpj as fits_read_grppar_lng;
pub use crate::ffggpjj as fits_read_grppar_lnglng;
pub use crate::ffggpk as fits_read_grppar_int;
pub use crate::ffggpsb as fits_read_grppar_sbyt;
pub use crate::ffggpui as fits_read_grppar_usht;
pub use crate::ffggpuj as fits_read_grppar_ulng;
pub use crate::ffggpujj as fits_read_grppar_ulnglng;
pub use crate::ffggpuk as fits_read_grppar_uint;

pub use crate::ffgpf as fits_read_imgnull;
pub use crate::ffgpv as fits_read_img;
pub use crate::ffgpvb as fits_read_img_byt;
pub use crate::ffgpvd as fits_read_img_dbl;
pub use crate::ffgpve as fits_read_img_flt;
pub use crate::ffgpvi as fits_read_img_sht;
pub use crate::ffgpvj as fits_read_img_lng;
pub use crate::ffgpvjj as fits_read_img_lnglng;
pub use crate::ffgpvk as fits_read_img_int;
pub use crate::ffgpvsb as fits_read_img_sbyt;
pub use crate::ffgpvui as fits_read_img_usht;
pub use crate::ffgpvuj as fits_read_img_ulng;
pub use crate::ffgpvujj as fits_read_img_ulnglng;
pub use crate::ffgpvuk as fits_read_img_uint;
pub use crate::ffgpxf as fits_read_pixnull;
pub use crate::ffgpxfll as fits_read_pixnullll;
pub use crate::ffgpxv as fits_read_pix;
pub use crate::ffgpxvll as fits_read_pixll;

pub use crate::ffgpfb as fits_read_imgnull_by;
pub use crate::ffgpfd as fits_read_imgnull_db;
pub use crate::ffgpfe as fits_read_imgnull_fl;
pub use crate::ffgpfi as fits_read_imgnull_sh;
pub use crate::ffgpfj as fits_read_imgnull_ln;
pub use crate::ffgpfjj as fits_read_imgnull_lngln;
pub use crate::ffgpfk as fits_read_imgnull_in;
pub use crate::ffgpfsb as fits_read_imgnull_sby;
pub use crate::ffgpfui as fits_read_imgnull_ush;
pub use crate::ffgpfuj as fits_read_imgnull_uln;
pub use crate::ffgpfujj as fits_read_imgnull_ulngln;
pub use crate::ffgpfuk as fits_read_imgnull_uin;

pub use crate::ffg2db as fits_read_2d_byt;
pub use crate::ffg2dd as fits_read_2d_dbl;
pub use crate::ffg2de as fits_read_2d_flt;
pub use crate::ffg2di as fits_read_2d_sht;
pub use crate::ffg2dj as fits_read_2d_lng;
pub use crate::ffg2djj as fits_read_2d_lnglng;
pub use crate::ffg2dk as fits_read_2d_int;
pub use crate::ffg2dsb as fits_read_2d_sbyt;
pub use crate::ffg2dui as fits_read_2d_usht;
pub use crate::ffg2duj as fits_read_2d_ulng;
pub use crate::ffg2dujj as fits_read_2d_ulnglng;
pub use crate::ffg2duk as fits_read_2d_uint;

pub use crate::ffg3db as fits_read_3d_byt;
pub use crate::ffg3dd as fits_read_3d_dbl;
pub use crate::ffg3de as fits_read_3d_flt;
pub use crate::ffg3di as fits_read_3d_sht;
pub use crate::ffg3dj as fits_read_3d_lng;
pub use crate::ffg3djj as fits_read_3d_lnglng;
pub use crate::ffg3dk as fits_read_3d_int;
pub use crate::ffg3dsb as fits_read_3d_sbyt;
pub use crate::ffg3dui as fits_read_3d_usht;
pub use crate::ffg3duj as fits_read_3d_ulng;
pub use crate::ffg3dujj as fits_read_3d_ulnglng;
pub use crate::ffg3duk as fits_read_3d_uint;

pub use crate::ffgsv as fits_read_subset;
pub use crate::ffgsvb as fits_read_subset_byt;
pub use crate::ffgsvd as fits_read_subset_dbl;
pub use crate::ffgsve as fits_read_subset_flt;
pub use crate::ffgsvi as fits_read_subset_sht;
pub use crate::ffgsvj as fits_read_subset_lng;
pub use crate::ffgsvjj as fits_read_subset_lnglng;
pub use crate::ffgsvk as fits_read_subset_int;
pub use crate::ffgsvsb as fits_read_subset_sbyt;
pub use crate::ffgsvui as fits_read_subset_usht;
pub use crate::ffgsvuj as fits_read_subset_ulng;
pub use crate::ffgsvujj as fits_read_subset_ulnglng;
pub use crate::ffgsvuk as fits_read_subset_uint;

pub use crate::ffgsfb as fits_read_subsetnull_by;
pub use crate::ffgsfd as fits_read_subsetnull_db;
pub use crate::ffgsfe as fits_read_subsetnull_fl;
pub use crate::ffgsfi as fits_read_subsetnull_sh;
pub use crate::ffgsfj as fits_read_subsetnull_ln;
pub use crate::ffgsfjj as fits_read_subsetnull_lngln;
pub use crate::ffgsfk as fits_read_subsetnull_in;
pub use crate::ffgsfsb as fits_read_subsetnull_sby;
pub use crate::ffgsfui as fits_read_subsetnull_ush;
pub use crate::ffgsfuj as fits_read_subsetnull_uln;
pub use crate::ffgsfujj as fits_read_subsetnull_ulngln;
pub use crate::ffgsfuk as fits_read_subsetnull_uin;

pub use crate::fits_comp_img as fits_compress_im;
pub use crate::fits_copy_image_section as ffcpim;
pub use crate::fits_decomp_img as fits_decompress_im;

pub use crate::ffgcf as fits_read_colnull;
pub use crate::ffgcv as fits_read_col;
pub use crate::ffgcvb as fits_read_col_byt;
pub use crate::ffgcvc as fits_read_col_cmp;
pub use crate::ffgcvd as fits_read_col_dbl;
pub use crate::ffgcve as fits_read_col_flt;
pub use crate::ffgcvi as fits_read_col_sht;
pub use crate::ffgcvj as fits_read_col_lng;
pub use crate::ffgcvjj as fits_read_col_lnglng;
pub use crate::ffgcvk as fits_read_col_int;
pub use crate::ffgcvl as fits_read_col_log;
pub use crate::ffgcvm as fits_read_col_dblcm;
pub use crate::ffgcvs as fits_read_col_str;
pub use crate::ffgcvsb as fits_read_col_sbyt;
pub use crate::ffgcvui as fits_read_col_usht;
pub use crate::ffgcvuj as fits_read_col_ulng;
pub use crate::ffgcvujj as fits_read_col_ulnglng;
pub use crate::ffgcvuk as fits_read_col_uint;
pub use crate::ffgcx as fits_read_col_bit;
pub use crate::ffgcxui as fits_read_col_bit_ush;
pub use crate::ffgcxuk as fits_read_col_bit_uin;

pub use crate::ffgcfb as fits_read_colnull_byt;
pub use crate::ffgcfc as fits_read_colnull_cmp;
pub use crate::ffgcfd as fits_read_colnull_dbl;
pub use crate::ffgcfe as fits_read_colnull_flt;
pub use crate::ffgcfi as fits_read_colnull_sht;
pub use crate::ffgcfj as fits_read_colnull_lng;
pub use crate::ffgcfjj as fits_read_colnull_lnglng;
pub use crate::ffgcfk as fits_read_colnull_int;
pub use crate::ffgcfl as fits_read_colnull_log;
pub use crate::ffgcfm as fits_read_colnull_dblcm;
pub use crate::ffgcfs as fits_read_colnull_str;
pub use crate::ffgcfsb as fits_read_colnull_sbyt;
pub use crate::ffgcfui as fits_read_colnull_usht;
pub use crate::ffgcfuj as fits_read_colnull_ulng;
pub use crate::ffgcfujj as fits_read_colnull_ulnglng;
pub use crate::ffgcfuk as fits_read_colnull_uint;

pub use crate::ffgdes as fits_read_descrip;
pub use crate::ffgdesll as fits_read_descriptl;
pub use crate::ffgdess as fits_read_descript;
pub use crate::ffgdessll as fits_read_descriptsl;
pub use crate::ffgtbb as fits_read_tblbytes;

pub use crate::ffpgpb as fits_write_grppar_by;
pub use crate::ffpgpd as fits_write_grppar_db;
pub use crate::ffpgpe as fits_write_grppar_fl;
pub use crate::ffpgpi as fits_write_grppar_sh;
pub use crate::ffpgpj as fits_write_grppar_ln;
pub use crate::ffpgpjj as fits_write_grppar_lngln;
pub use crate::ffpgpk as fits_write_grppar_in;
pub use crate::ffpgpsb as fits_write_grppar_sby;
pub use crate::ffpgpui as fits_write_grppar_ush;
pub use crate::ffpgpuj as fits_write_grppar_uln;
pub use crate::ffpgpujj as fits_write_grppar_ulngln;
pub use crate::ffpgpuk as fits_write_grppar_uin;

pub use crate::ffppr as fits_write_img;
pub use crate::ffpprb as fits_write_img_byt;
pub use crate::ffpprd as fits_write_img_dbl;
pub use crate::ffppre as fits_write_img_flt;
pub use crate::ffppri as fits_write_img_sht;
pub use crate::ffpprj as fits_write_img_lng;
pub use crate::ffpprjj as fits_write_img_lnglng;
pub use crate::ffpprk as fits_write_img_int;
pub use crate::ffpprsb as fits_write_img_sbyt;
pub use crate::ffpprui as fits_write_img_usht;
pub use crate::ffppruj as fits_write_img_ulng;
pub use crate::ffpprujj as fits_write_img_ulnglng;
pub use crate::ffppruk as fits_write_img_uint;
pub use crate::ffppx as fits_write_pix;
pub use crate::ffppxll as fits_write_pixll;
pub use crate::ffppxn as fits_write_pixnull;
pub use crate::ffppxnll as fits_write_pixnullll;

pub use crate::ffppn as fits_write_imgnull;
pub use crate::ffppnb as fits_write_imgnull_by;
pub use crate::ffppnd as fits_write_imgnull_db;
pub use crate::ffppne as fits_write_imgnull_fl;
pub use crate::ffppni as fits_write_imgnull_sh;
pub use crate::ffppnj as fits_write_imgnull_ln;
pub use crate::ffppnjj as fits_write_imgnull_lngln;
pub use crate::ffppnk as fits_write_imgnull_in;
pub use crate::ffppnsb as fits_write_imgnull_sby;
pub use crate::ffppnui as fits_write_imgnull_ush;
pub use crate::ffppnuj as fits_write_imgnull_uln;
pub use crate::ffppnujj as fits_write_imgnull_ulngln;
pub use crate::ffppnuk as fits_write_imgnull_uin;

pub use crate::ffpprn as fits_write_null_im;
pub use crate::ffppru as fits_write_img_nul;

pub use crate::ffp2db as fits_write_2d_byt;
pub use crate::ffp2dd as fits_write_2d_dbl;
pub use crate::ffp2de as fits_write_2d_flt;
pub use crate::ffp2di as fits_write_2d_sht;
pub use crate::ffp2dj as fits_write_2d_lng;
pub use crate::ffp2djj as fits_write_2d_lnglng;
pub use crate::ffp2dk as fits_write_2d_int;
pub use crate::ffp2dsb as fits_write_2d_sbyt;
pub use crate::ffp2dui as fits_write_2d_usht;
pub use crate::ffp2duj as fits_write_2d_ulng;
pub use crate::ffp2dujj as fits_write_2d_ulnglng;
pub use crate::ffp2duk as fits_write_2d_uint;

pub use crate::ffp3db as fits_write_3d_byt;
pub use crate::ffp3dd as fits_write_3d_dbl;
pub use crate::ffp3de as fits_write_3d_flt;
pub use crate::ffp3di as fits_write_3d_sht;
pub use crate::ffp3dj as fits_write_3d_lng;
pub use crate::ffp3djj as fits_write_3d_lnglng;
pub use crate::ffp3dk as fits_write_3d_int;
pub use crate::ffp3dsb as fits_write_3d_sbyt;
pub use crate::ffp3dui as fits_write_3d_usht;
pub use crate::ffp3duj as fits_write_3d_ulng;
pub use crate::ffp3dujj as fits_write_3d_ulnglng;
pub use crate::ffp3duk as fits_write_3d_uint;

pub use crate::ffpss as fits_write_subset;
pub use crate::ffpssb as fits_write_subset_byt;
pub use crate::ffpssd as fits_write_subset_dbl;
pub use crate::ffpsse as fits_write_subset_flt;
pub use crate::ffpssi as fits_write_subset_sht;
pub use crate::ffpssj as fits_write_subset_lng;
pub use crate::ffpssjj as fits_write_subset_lnglng;
pub use crate::ffpssk as fits_write_subset_int;
pub use crate::ffpsssb as fits_write_subset_sbyt;
pub use crate::ffpssui as fits_write_subset_usht;
pub use crate::ffpssuj as fits_write_subset_ulng;
pub use crate::ffpssujj as fits_write_subset_ulnglng;
pub use crate::ffpssuk as fits_write_subset_uint;

pub use crate::ffpcl as fits_write_col;
pub use crate::ffpclb as fits_write_col_byt;
pub use crate::ffpclc as fits_write_col_cmp;
pub use crate::ffpcld as fits_write_col_dbl;
pub use crate::ffpcle as fits_write_col_flt;
pub use crate::ffpcli as fits_write_col_sht;
pub use crate::ffpclj as fits_write_col_lng;
pub use crate::ffpcljj as fits_write_col_lnglng;
pub use crate::ffpclk as fits_write_col_int;
pub use crate::ffpcll as fits_write_col_log;
pub use crate::ffpclm as fits_write_col_dblcmp;
pub use crate::ffpcls as fits_write_col_str;
pub use crate::ffpclsb as fits_write_col_sbyt;
pub use crate::ffpclu as fits_write_col_null;
pub use crate::ffpclui as fits_write_col_usht;
pub use crate::ffpcluj as fits_write_col_ulng;
pub use crate::ffpclujj as fits_write_col_ulnglng;
pub use crate::ffpcluk as fits_write_col_uint;
pub use crate::ffpclx as fits_write_col_bit;
pub use crate::ffprwu as fits_write_nulrows;
pub use crate::ffprwu as fits_write_nullrows;

pub use crate::ffpcn as fits_write_colnul;
pub use crate::ffpcnb as fits_write_colnull_by;
pub use crate::ffpcnd as fits_write_colnull_db;
pub use crate::ffpcne as fits_write_colnull_fl;
pub use crate::ffpcni as fits_write_colnull_sh;
pub use crate::ffpcnj as fits_write_colnull_ln;
pub use crate::ffpcnjj as fits_write_colnull_lngln;
pub use crate::ffpcnk as fits_write_colnull_in;
pub use crate::ffpcnl as fits_write_colnull_lo;
pub use crate::ffpcns as fits_write_colnull_st;
pub use crate::ffpcnsb as fits_write_colnull_sby;
pub use crate::ffpcnui as fits_write_colnull_ush;
pub use crate::ffpcnuj as fits_write_colnull_uln;
pub use crate::ffpcnujj as fits_write_colnull_ulngln;
pub use crate::ffpcnuk as fits_write_colnull_uin;

pub use crate::ffgextn as fits_read_ext;
pub use crate::ffpextn as fits_write_ex;

pub use crate::ffcmph as fits_compress_heap;
pub use crate::ffpdes as fits_write_descript;
pub use crate::fftheap as fits_test_heap;

pub use crate::ffccls as fits_copy_cols;
pub use crate::ffcpcl as fits_copy_col;
pub use crate::ffcprw as fits_copy_rows;
pub use crate::ffdcol as fits_delete_col;
pub use crate::ffdrow as fits_delete_rows;
pub use crate::ffdrrg as fits_delete_rowrang;
pub use crate::ffdrws as fits_delete_rowlis;
pub use crate::ffdrwsll as fits_delete_rowlistl;
pub use crate::fficls as fits_insert_cols;
pub use crate::fficol as fits_insert_col;
pub use crate::ffirow as fits_insert_rows;
pub use crate::ffmvec as fits_modify_vector_len;
pub use crate::ffptbb as fits_write_tblbytes;

pub use crate::ffgics as fits_read_img_coor;
pub use crate::ffgicsa as fits_read_img_coord_versio;
pub use crate::ffgtcs as fits_read_tbl_coor;
pub use crate::ffwldp as fits_pix_to_worl;
pub use crate::ffxypx as fits_world_to_pi;

pub use crate::ffgiwcs as fits_get_image_wcs_key;
pub use crate::ffgtwcs as fits_get_table_wcs_key;

pub use crate::ffcalc as fits_calculator;
pub use crate::ffcalc_rng as fits_calculator_rng;
pub use crate::ffcrow as fits_calc_rows;
pub use crate::ffffrw as fits_find_first_row;
pub use crate::fffrow as fits_find_rows;
pub use crate::fffrwc as fits_find_rows_cmp;
pub use crate::ffsrow as fits_select_rows;
pub use crate::fftexp as fits_test_expr;

pub use crate::ffgtam as fits_add_group_member;
pub use crate::ffgtch as fits_change_group;
pub use crate::ffgtcm as fits_compact_group;
pub use crate::ffgtcp as fits_copy_group;
pub use crate::ffgtcr as fits_create_group;
pub use crate::ffgtis as fits_insert_group;
pub use crate::ffgtmg as fits_merge_groups;
pub use crate::ffgtnm as fits_get_num_members;
pub use crate::ffgtop as fits_open_group;
pub use crate::ffgtrm as fits_remove_group;
pub use crate::ffgtvf as fits_verify_group;

pub use crate::ffgmcp as fits_copy_member;
pub use crate::ffgmng as fits_get_num_groups;
pub use crate::ffgmop as fits_open_member;
pub use crate::ffgmrm as fits_remove_member;
pub use crate::ffgmtf as fits_transfer_member;

pub use crate::ffchtps as fits_cleanup_https;
pub use crate::ffihtps as fits_init_https;
pub use crate::ffvhtps as fits_verbose_https;

pub use crate::ffgtmo as fits_get_timeout;
pub use crate::ffshdwn as fits_show_download_progress;
pub use crate::ffstmo as fits_set_timeout;

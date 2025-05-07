#include <inttypes.h>
#include <stdlib.h>

typedef struct OggOpusFile OggOpusFile;
typedef int64_t ogg_int64_t;
typedef int16_t opus_int16;
typedef struct
{
    unsigned char *body_data;
    long body_storage;
    long body_fill;
    long body_returned;
    int *lacing_vals;
    ogg_int64_t *granule_vals;
    long lacing_storage;
    long lacing_fill;
    long lacing_packet;
    long lacing_returned;
    unsigned char header[282];
    int header_fill;
    int e_o_s;
    int b_o_s;
    long serialno;
    long pageno;
    ogg_int64_t packetno;
    ogg_int64_t granulepos;
} ogg_stream_state;
typedef struct
{
    unsigned char *header;
    long header_len;
    unsigned char *body;
    long body_len;
} ogg_page;
typedef struct
{
    unsigned char *packet;
    long bytes;
    long b_o_s;
    long e_o_s;
    ogg_int64_t granulepos;
    ogg_int64_t packetno;
} ogg_packet;
extern int ogg_stream_init(ogg_stream_state *os, int serialno);
extern int ogg_stream_packetin(ogg_stream_state *os, ogg_packet *op);
extern int ogg_stream_pageout(ogg_stream_state *os, ogg_page *og);
extern int ogg_stream_flush(ogg_stream_state *os, ogg_page *og);
extern OggOpusFile *op_open_memory(const unsigned char *_data,
                                   size_t _size, int *_error);
extern int op_read_stereo(OggOpusFile *_of,
                          opus_int16 *_pcm, int _buf_size);
extern int opus_packet_get_nb_samples(const unsigned char packet[], int len, int Fs);
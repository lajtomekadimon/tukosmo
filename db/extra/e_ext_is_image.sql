
CREATE OR REPLACE FUNCTION e_ext_is_image(
    ext_value TEXT
)

RETURNS BOOL

LANGUAGE SQL
IMMUTABLE
RETURNS NULL ON NULL INPUT
PARALLEL SAFE

AS $$

SELECT ext_value IN (
    -- Raster formats
    ------------------------------------------------
    -- JPEG
    'jpg', 'jpeg', 'jfi', 'jpe', 'jif', 'jfif',
    -- JPEG 2000
    --'jp2', 'j2k', 'jpf', 'jpx', 'jpm', 'mj2',
    -- HEIF
    'heif', 'heic',
    -- PNG
    'png',
    -- GIF
    'gif',
    -- WebP
    'webp',
    -- TIFF
    'tiff', 'tif',
    -- Bitmap
    'bmp',
    -- RAW
    --'raw', 'arw', 'cr', 'cr2', 'rw2', 'nrw', 'k25', 'nef',
    --'orf', 'sr2',
    -- Vector formats / Other
    ------------------------------------------------
    -- SVG
    'svg', 'svgz'
    -- EPS
    --'eps',
    -- AI
    --'ai',
)

$$;

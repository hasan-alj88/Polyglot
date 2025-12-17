# Multi-Step Pipeline Examples

**Version:** 0.0.2  
**Working Code:** Complete ETL and Processing Pipelines
**Last Updated:** 2025-12-02

---

## Example 1: ETL Workflow

Extract-Transform-Load pipeline using Python, Rust, and Go.

### `ETLPipeline.pg`

```polyglot
[@] @Local::ETLExample:1.0.0.0
[#] 1
[X]


[#] #RawData
[<] <records:pg.array{pg\serial
[<] <source:pg.string
[<] <timestamp:pg.dt
[X]


[#] #TransformedData
[<] <records:pg.array{pg\serial
[<] <metadata:pg.serial
[X]


[|] ExtractData
[i] .api_endpoint:pg.string
[i] .auth_token:pg.string
[t] |T.Call
[W] RT.Python"fetch_api.py"
[o] .raw_data: #RawData
[X]


[|] TransformData
[i] .raw_data: #RawData
[t] |T.Call
[W] RT.Rust"transform::transform_records"
[o] .transformed: #TransformedData
[X]


[|] LoadData
[i] .data: #TransformedData
[i] .target_db:pg.string
[t] |T.Call
[W] RT.Go"main.LoadToDatabase"
[o] .rows_inserted:pg.int
[X]


[|] ETLWorkflow
[i] .api_endpoint:pg.string
[i] .auth_token:pg.string
[i] .target_db:pg.string
[t] |T.DT.Every"1h"
// |W.Polyglot.Scope is IMPLICIT (manages variable lifecycle

[r] |ExtractData
[<] <api_endpoint:pg.string << .api_endpoint
[<] <auth_token:pg.string << .auth_token
[>] >raw_data: #RawData >> .extracted

[r] |TransformData
[<] <raw_data: #RawData << .extracted
[>] >transformed: #TransformedData >> .clean_data

[r] |LoadData
[<] <data: #TransformedData << .clean_data
[<] <target_db:pg.string << .target_db
[>] >rows_inserted:pg.int >> .count

[o] .count:pg.int
[X]
```

---

## Example 2: Parallel Image Processing

Parallel image processing with Rust.

### `ImagePipeline.pg`

```polyglot
[@] @Local::ImageProcessing:1.0.0.0
[#] 1
[X]


[|] LoadImage
[i] .path:pg.path
[t] |T.Call
[W] RT.Rust"image::load_image"
[o] .image_data:pg.serial
[X]


[|] ResizeImage
[i] .image_data:pg.serial
[i] .width:pg.int
[i] .height:pg.int
[t] |T.Call
[W] RT.Rust"image::resize"
[o] .resized:pg.serial
[X]


[|] SaveImage
[i] .image_data:pg.serial
[i] .output_path:pg.path
[t] |T.Call
[W] RT.Rust"image::save_image"
[o] .saved_path:pg.path
[X]


[|] ProcessImagesParallel
[i] .input_paths:pg.array{pg\path
[i] .output_dir:pg.path
[i] .target_width:pg.int
[i] .target_height:pg.int
[t] |T.Call
// |W.Polyglot.Scope is IMPLICIT (manages variable lifecycle

[p] ~ForEach
[<] .input_paths
[>] .path
[~][r] |LoadImage
[~][<] <path:pg.path << .path
[~][>] >image_data:pg.serial >> .loaded
[~]
[~][r] |ResizeImage
[~][<] <image_data:pg.serial << .loaded
[~][<] <width:pg.int << .target_width
[~][<] <height:pg.int << .target_height
[~][>] >resized:pg.serial >> .resized_image
[~]
[~][r] .output_path:pg.path << "{.output_dir/{.path"
[~][r] |SaveImage
[~][<] <image_data:pg.serial << .resized_image
[~][<] <output_path:pg.path << .output_path
[~][>] >saved_path:pg.path >> .final_path
[~]
[~][Y] ~Y.IntoArray
[~][<] .final_path
[~][>] .processed_paths
[~]

[o] .processed_paths:pg.array{pg\path
[X]
```

---

**Next:** [Error Handling Patterns →](error-handling-patterns.md

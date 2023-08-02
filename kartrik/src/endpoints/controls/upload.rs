use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::tokio::fs::copy;

#[derive(FromForm)]
pub(crate) struct UploadFormData<'f> {
    file: TempFile<'f>,
}

#[post(
    "/api/v1/controls/upload",
    format = "multipart/form-data",
    data = "<form>"
)]
pub(crate) async fn upload(form: Form<UploadFormData<'_>>) -> std::io::Result<()> {
    println!("form.file = {:?}", form.file);
    let file_name = "test.csv";

    println!("destination = {}", file_name);
    println!("length = {} bytes", form.file.len());

    let source_path = form.file.path().to_owned();
    copy(source_path.unwrap(), file_name).await?;

    Ok(())
}

use crate::generate_endpoint_roles;
use crate::helpers::surrealdb::control::add_control;
use crate::models::control::Control;
use crate::models::{role::Role, user::User};
use csv::{ReaderBuilder, StringRecord, StringRecordsIter};
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::status::{self, BadRequest};
use rocket::{form::Form, State};
use std::collections::HashSet;
use std::io;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

#[derive(FromForm)]
pub(crate) struct UploadFormData<'f> {
    file: TempFile<'f>,
}

generate_endpoint_roles!(AddControlsRole, { Role::EditControls });

#[post("/upload", format = "multipart/form-data", data = "<form>")]
pub(crate) async fn upload(
    form: Form<UploadFormData<'_>>,
    _authz: AddControlsRole,
    db: &State<Surreal<Client>>,
) -> Result<String, status::BadRequest<String>> {
    println!("Got a new file {:?}", form.file);

    let path = form.file.path().expect("The temporary file should exist");
    let mut reader = ReaderBuilder::new()
        .from_path(path)
        .expect("Is a valid CSV");
    let header_record_opt = reader.headers();

    match header_record_opt {
        Ok(first_record) => {
            // TODO: change this to use a schema an pass it later to controlFromRecord
            let frist_field = first_record.get(0);
            let second_field = first_record.get(1);
            println!("{frist_field:?}, {second_field:?}");
            let is_header_valid =
                frist_field == Some("Title") && second_field == Some("Description");

            if is_header_valid {
                parse_records(reader.records(), db).await
            } else {
                Err(BadRequest(Some("The first column of the CSV must be \"Title\", and the second one \"Description\"".to_string())))
            }
        }
        Err(err) => {
            println!("Could not parse the first record of the file: {err:?}");
            Err(BadRequest(Some(format!(
                "Could not parse the first record of the file. Ensure it is a valid csv"
            ))))
        }
    }
}

async fn parse_records(
    records: StringRecordsIter<'_, impl io::Read>,
    db: &State<Surreal<Client>>,
) -> Result<String, status::BadRequest<String>> {
    let mut total_records = 0;
    let mut total_controls = 0;
    for record_or_err in records {
        total_records += 1;
        match record_or_err {
            Ok(record) => {
                let opt_control = control_from_record(record);
                match opt_control {
                    Some(control) => {
                        //Add to db
                        match add_control(control, db).await {
                            Ok(_) => total_controls += 1,
                            Err(err) => println!("Could not add control from record #{total_records} to the database {err:?}"),
                        }
                    }
                    None => println!("Could not create control from record #{total_records}"),
                }
            }
            Err(err) => println!("Could not extract record #{total_records}: {err:?}"),
        }
    }

    Ok(format!(
        "Added {total_controls} controls over {total_records} records present in the csv"
    ))
}

fn control_from_record(record: StringRecord) -> Option<Control> {
    let title_opt = record.get(0);
    let description_opt = record.get(1);
    if let Some(title) = title_opt {
        if let Some(desc) = description_opt {
            return Some(Control::new(title, desc));
        }
    }
    None
}

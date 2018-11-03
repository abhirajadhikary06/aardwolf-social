use rocket::request::Form;

use aardwolf_models::user::PermissionError;
use aardwolf_types::forms::personas::{
    CheckDeletePersonaPermission, CreatePersona, DeletePersona, FetchPersona, PersonaCreationFail,
    PersonaCreationForm, PersonaDeletionFail, PersonaLookupError, ValidatePersonaCreationForm,
};
use types::user::SignedInUser;
use DbConn;

use crate::action::{DbActionWrapper, ValidateWrapper};

#[get("/new")]
fn new(_user: SignedInUser) -> String {
    format!("placeholder")
}

#[derive(Clone, Debug, Fail)]
pub enum PersonaCreateError {
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "User does not have permission to create a persona")]
    Permission,
    #[fail(display = "Submitted form is invalid")]
    Form,
}

impl From<PersonaCreationFail> for PersonaCreateError {
    fn from(e: PersonaCreationFail) -> Self {
        match e {
            PersonaCreationFail::Validation => PersonaCreateError::Form,
            PersonaCreationFail::Permission => PersonaCreateError::Permission,
            PersonaCreationFail::Database => PersonaCreateError::Database,
        }
    }
}

#[post("/create", data = "<form>")]
fn create(
    user: SignedInUser,
    form: Form<PersonaCreationForm>,
    db: DbConn,
) -> Result<String, PersonaCreateError> {
    let _ = perform!(
        &db,
        form.into_inner(),
        PersonaCreateError,
        [
            (ValidateWrapper<_, _, _> => ValidatePersonaCreationForm),
            (DbActionWrapper<_, _, _> => CreatePersona::new(user.0)),
        ]
    )?;

    Ok(format!("Created!"))
}

#[derive(Clone, Debug, Fail)]
pub enum PersonaDeleteError {
    #[fail(display = "Error talking to db actor")]
    Mailbox,
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "Error confirming account: {}", _0)]
    Delete(#[cause] PersonaDeletionFail),
}

impl From<PersonaDeletionFail> for PersonaDeleteError {
    fn from(e: PersonaDeletionFail) -> Self {
        PersonaDeleteError::Delete(e)
    }
}

impl From<PersonaLookupError> for PersonaDeleteError {
    fn from(e: PersonaLookupError) -> Self {
        PersonaDeleteError::Delete(e.into())
    }
}

impl From<PermissionError> for PersonaDeleteError {
    fn from(e: PermissionError) -> Self {
        PersonaDeleteError::Delete(e.into())
    }
}

#[get("/delete/<id>")]
fn delete(user: SignedInUser, id: i32, db: DbConn) -> Result<String, PersonaDeleteError> {
    let _ = perform!(
        &db,
        id,
        PersonaDeleteError,
        [
            (DbActionWrapper<_, _, _> => FetchPersona),
            (DbActionWrapper<_, _, _> => CheckDeletePersonaPermission::new(user.0)),
            (DbActionWrapper<_, _, _> => DeletePersona),
        ]
    )?;

    Ok(format!("Deleted!"))
}

#[get("/switch/<switch_persona>")]
fn switch(_user: SignedInUser, switch_persona: i32) -> String {
    format!("placeholder, {}", switch_persona)
}

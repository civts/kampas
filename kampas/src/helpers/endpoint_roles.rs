#[macro_export]
/// This macro generates a struct that implements the `FromRequest` trait for Rocket framework.
///
/// The generated struct checks if the user has all the required roles before allowing access to the endpoint.
macro_rules! generate_endpoint_roles {
    ($struct_name:ident, { $($role:expr),* }) => {

        pub(crate) struct $struct_name {}

        #[rocket::async_trait]
        impl<'r> FromRequest<'r> for $struct_name {
            type Error = &'static str;

            async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
                let required_roles = vec![$($role),*];

                let user_from_req = req.guard::<User>().await;
                match user_from_req {
                    Outcome::Success(user) => {
                        let required_roles_set: HashSet<_> = required_roles.into_iter().collect();
                        let user_roles_set: HashSet<_> = user.roles.into_iter().collect();

                        let has_all_roles = required_roles_set.is_subset(&user_roles_set);

                        if has_all_roles {
                            Outcome::Success($struct_name {})
                        } else {
                            Outcome::Error((
                                Status::Forbidden,
                                Status::Forbidden
                                    .reason()
                                    .expect("Each status code should have a reason"),
                            ))
                        }
                    }
                    _ => Outcome::Error((
                        Status::Unauthorized,
                        Status::Unauthorized
                            .reason()
                            .expect("Each status code should have a reason"),
                    )),
                }
            }
        }
    };
}

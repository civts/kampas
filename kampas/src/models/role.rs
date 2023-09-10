use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(crate = "rocket::serde")]
/// The roles a user can have. A user may have any number of these roles.
pub(crate) enum Role {
    /// This user is an administrator
    #[serde(rename = "Admin")]
    Admin,

    /// Can generate the one-time codes required to register new accounts
    #[serde(rename = "GenerateSignupCodes")]
    GenerateSignupCodes,

    /// Can add new controls
    #[serde(rename = "CreateControls")]
    CreateControls,

    /// Can see which controls are present in the database
    #[serde(rename = "GetControls")]
    GetControls,

    /// Can modify the controls -e.g., description, title, tags, etc.-.
    /// This also grants permission to delete them.
    #[serde(rename = "EditControls")]
    EditControls,

    /// Can create new tags
    #[serde(rename = "CreateTags")]
    CreateTags,

    /// Can see which tags are present in the database
    #[serde(rename = "GetTags")]
    GetTags,

    /// Can modify the tags. This also grants permission to delete them.
    #[serde(rename = "EditTags")]
    EditTags,

    /// Can create new measures
    #[serde(rename = "CreateMeasures")]
    CreateMeasures,

    /// Can see which measures are present in the database
    #[serde(rename = "GetMeasures")]
    GetMeasures,

    /// Can modify the measures. This also grants permission to delete them.
    #[serde(rename = "EditMeasures")]
    EditMeasures,

    /// Can associate (existing) measures to (existing) controls
    #[serde(rename = "AssociateMeasures")]
    AssociateMeasures,

    /// Can create a new ranking
    #[serde(rename = "CreateRanking")]
    CreateRanking,

    /// Can see the rankings
    #[serde(rename = "GetRankings")]
    GetRankings,

    /// Can see the audit trace for all regular users
    #[serde(rename = "SeeUserActivity")]
    SeeUserActivity,

    /// Can clear the audit trace for all regular users
    #[serde(rename = "ClearUserActivity")]
    ClearUserActivity,

    /// Can see the audit trace for all admin users
    #[serde(rename = "SeeAdminActivity")]
    SeeAdminActivity,

    /// Can clear the audit trace for all admin users
    #[serde(rename = "ClearAdminActivity")]
    ClearAdminActivity,

    /// Can assign new roles to users, but no more than the ones assigned to this account
    #[serde(rename = "EditUserRoles")]
    EditUserRoles,

    /// Can choose when the auth tokens for regular users should expire
    #[serde(rename = "SetUserAuthTokenDuration")]
    SetUserAuthTokenDuration,

    /// Can choose when the auth tokens for administrative users should expire
    #[serde(rename = "SetAdminAuthTokenDuration")]
    SetAdminAuthTokenDuration,
}

use crate::{
    error::FriendTechAppError,
    handlers,
    msg::{
        FriendTechAppExecuteMsg, FriendTechAppInstantiateMsg, FriendTechAppMigrateMsg,
        FriendTechAppQueryMsg,
    },
    replies::{self, INSTANTIATE_REPLY_ID},
    FRIEND_TECH_APP_ID, FRIEND_TECH_APP_VERSION,
};

use abstract_app::AppContract;
use cosmwasm_std::Response;

/// The type of the result returned by your app's entry points.
pub type FriendTechAppResult<T = Response> = Result<T, FriendTechAppError>;

/// The type of the app that is used to build your app and access the Abstract SDK features.
pub type FriendTechApp = AppContract<
    FriendTechAppError,
    FriendTechAppInstantiateMsg,
    FriendTechAppExecuteMsg,
    FriendTechAppQueryMsg,
    FriendTechAppMigrateMsg,
>;

const FRIEND_TECH_APP: FriendTechApp =
    FriendTechApp::new(FRIEND_TECH_APP_ID, FRIEND_TECH_APP_VERSION, None)
        .with_instantiate(handlers::instantiate_handler)
        .with_execute(handlers::execute_handler)
        .with_query(handlers::query_handler)
        .with_migrate(handlers::migrate_handler)
        .with_dependencies(&[])
        .with_replies(&[(INSTANTIATE_REPLY_ID, replies::instantiate_reply)]);

// Export handlers
#[cfg(feature = "export")]
abstract_app::export_endpoints!(FRIEND_TECH_APP, FriendTechApp);

abstract_app::cw_orch_interface!(FRIEND_TECH_APP, FriendTechApp, Friendtech);

// TODO: add to docmuentation
// https://linear.app/abstract-sdk/issue/ABS-414/add-documentation-on-dependencycreation-trait
#[cfg(not(target_arch = "wasm32"))]
impl<Chain: cw_orch::environment::CwEnv> abstract_interface::DependencyCreation
    for crate::Friendtech<Chain>
{
    type DependenciesConfig = cosmwasm_std::Empty;
}

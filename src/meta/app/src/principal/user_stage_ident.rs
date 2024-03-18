// Copyright 2021 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::tenant_key::TIdent;

/// Define the meta-service key for a stage.
pub type StageIdent = TIdent<kvapi_impl::Resource>;

mod kvapi_impl {
    use std::fmt::Display;

    use databend_common_exception::ErrorCode;
    use databend_common_meta_kvapi::kvapi;

    use crate::principal::StageInfo;
    use crate::tenant::Tenant;
    use crate::tenant_key::TenantResource;

    pub struct Resource;
    impl TenantResource for Resource {
        const PREFIX: &'static str = "__fd_stages";
        type ValueType = StageInfo;

        type UnknownError = ErrorCode;

        fn error_unknown<D: Display>(
            _tenant: &Tenant,
            _name: &str,
            _ctx: impl FnOnce() -> D,
        ) -> Self::UnknownError {
            todo!()
        }

        type ExistError = ErrorCode;

        fn error_exist<D: Display>(
            _tenant: &Tenant,
            _name: &str,
            _ctx: impl FnOnce() -> D,
        ) -> Self::ExistError {
            todo!()
        }
    }

    impl kvapi::Value for StageInfo {
        fn dependency_keys(&self) -> impl IntoIterator<Item = String> {
            []
        }
    }
}

#[cfg(test)]
mod tests {
    use databend_common_meta_kvapi::kvapi::Key;

    use crate::principal::user_stage_ident::StageIdent;
    use crate::tenant::Tenant;

    #[test]
    fn test_kvapi_key_for_stage_ident() {
        let tenant = Tenant::new("test");
        let stage = StageIdent::new(tenant, "stage");

        let key = stage.to_string_key();
        assert_eq!(key, "__fd_stages/test/stage");
        assert_eq!(stage, StageIdent::from_str_key(&key).unwrap());
    }
}
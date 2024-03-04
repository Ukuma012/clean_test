/*
エンドポイント
URL: /api/v1/register/complete/{invitation_token}
Method: POST

提供するサービスの概要
事前に所有権を確認するためにユーザーが入力したメールアドレスに送付されたトークンつきリンクによってアクセスしている必要がある。
ユーザーがpostしたパスワードをハッシュ化して、emailとともにDBに保存する。その後、session IDを発行する

引数
email: String型
pasword: String型

戻り値
ステータスコード: 200 OK
session ID(Cookieに格納すること)

エラー
パラメータの不正値エラー
・emailがString型でない
・パスワードがString型でない
ステータスコード： 400 Bad Request

誤った順序で呼び出されたエラー
・invitation_usecaseが実行された後に呼び出される必要がある。つまり、メールの所有権を確認した後でないといけない。
もしinvitation_usecaseよりも先に呼び出された場合、tokenを確認して登録されたものでないはずなので、401 Unauthorizedが返される。
    ステータスコード： 401 Unauthorized

認証・認可のエラー
・emailが所有権確認したものと異なる場合
・トークンのvalidationに失敗した場合
    失敗する可能性は二つ
        トークンが正しくない
        有効期限を過ぎている
    ステータスコード： 401 Unauthorized
 */
use async_trait::async_trait;

use crate::application::repositories::invitation_repository_abstract::InvitationRepositoryAbstract;
use crate::application::repositories::register_complete_repository_abstract::RegisterCompleteRepositoryAbstract;
use crate::application::usecases::interfaces::AbstractRegisterCompleteUseCase;
use crate::domain::user_entity::UserEntity;
use crate::error::AppError;

pub struct RegisterCompleteUseCase<'a> {
    email: &'a String,
    password: &'a String,
    invitation_token: &'a String,
    invitation_repository: &'a dyn InvitationRepositoryAbstract,
    register_complete_repository: &'a dyn RegisterCompleteRepositoryAbstract,
}

impl<'a> RegisterCompleteUseCase<'a> {
    pub fn new(
        email: &'a String,
        password: &'a String,
        invitation_token: &'a String,
        invitation_repository: &'a dyn InvitationRepositoryAbstract,
        register_complete_repository: &'a dyn RegisterCompleteRepositoryAbstract,
    ) -> Self {
        RegisterCompleteUseCase {
            email,
            password,
            invitation_token,
            invitation_repository,
            register_complete_repository,
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractRegisterCompleteUseCase<UserEntity> for RegisterCompleteUseCase<'a> {
    async fn register(&self) -> Result<UserEntity, AppError> {
        // tokenをvalidation
        let invitation_token = uuid::Uuid::parse_str(&self.invitation_token)?;
        let validate_result = self.invitation_repository.validate_invitation_token(invitation_token).await?;
        if !(validate_result) {
            Err(AppError::Unauthorized("Invalid Token".into()))
        } else {
            // パスワードのハッシュ化
            // emailとpasswordを保存
            let insertd_user = self.register_complete_repository.insert_user(self.email.to_string(), self.password.to_string()).await?;

            // sessionIDの発行
            // UserPresenterでsession IDを設定し、to_apiで詰め替える
            Ok(insertd_user)
        }
    }
}

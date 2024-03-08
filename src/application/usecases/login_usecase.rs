/**
 * エンドポイント
 * URL: /api/v1/login
 * Method: POST
 *
 * 提供するサービスの概要
 * 事前に会員登録したユーザーが入力したメールアドレスとパスワードを元に認可を行う。
 * 認可に成功したユーザーにはsessionIDを発行する。
 *
 * 引数
 * email: String型
 * password: String型
 *
 * 成功時の返り値
 * status code: 200OK
 * Cookie: session_id
 *
 * エラー
 * ・パラメータの不正値エラー
 *  emailがString型ではない
 *  passwordがString型ではない
 *  status code: 401 Unauthorized
 *
 * ・emailの登録がない
 *  status code: 401 Unauthorized
 *
 * ・パスワードが間違っている
 *  status code: 401 Unauthorized
 *
 * ・誤った順番で呼び出された
 *  register/completeのあとに呼び出される必要がある。つまり、会員登録が済んでいないといけない。
 *  認可で失敗する
 *  status code: 401 Unauthorized
 *
 */
use async_trait::async_trait;

use crate::application::repositories::login_repository_abstract::LoginRepositoryAbstract;
use crate::application::usecases::interfaces::AbstractLoginUseCase;
use crate::error::AppError;

pub struct LoginUseCase<'a> {
    email: &'a String,
    password: &'a String,
    login_repository: &'a dyn LoginRepositoryAbstract,
}

impl<'a> LoginUseCase<'a> {
    pub fn new(email: &'a String, password: &'a String, login_repository: &'a dyn LoginRepositoryAbstract) -> Self {
        LoginUseCase { email, password, login_repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractLoginUseCase for LoginUseCase<'a> {
    async fn login(&self) -> Result<(), AppError> {
        // email, passwordで認可
        self.login_repository.retrieval_user(self.email.to_string(), self.password.to_string()).await?;
        Ok(())
    }
}

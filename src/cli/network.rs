use clap::{Args, Subcommand};
use ctbox::network::{self, entity::User};

use super::{config, data};

#[derive(Subcommand)]
pub enum Command {
    /// 登录
    Login {
        #[command(flatten)]
        login_with_account: LoginWithAccount,
        #[command(flatten)]
        login_with_label: LoginWithLabel,
    },
    /// 登出
    Logout {},
    /// 查询
    Query {
        /// 账号
        #[arg(short, long)]
        account: Option<String>,
    },
    /// 加解密
    Encrypt {
        #[arg(short, long)]
        /// 是否为解密模式
        decrypt: bool,
        /// 源文
        source: String,
    },
}

#[derive(Args, Debug)]
#[group(required = false, multiple = true, conflicts_with = "LoginWithLabel")]
pub struct LoginWithAccount {
    /// 账号
    #[arg(short, long, requires = "password")]
    pub account: Option<String>,
    /// 密码
    #[arg(short, long, requires = "account")]
    pub password: Option<String>,
    /// 登录并保存账号
    #[arg(short, long, requires = "account", requires = "password")]
    pub save: Option<String>,
    /// 是否保存为默认账号
    #[arg(short, long, requires = "save")]
    pub default: bool,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = true, conflicts_with = "LoginWithAccount")]
pub struct LoginWithLabel {
    /// 读取并登录账号
    #[arg(short, long)]
    pub load: Option<String>,
}

pub fn login(
    data: &mut data::Network,
    _config: &config::Network,
    login_with_account: LoginWithAccount,
    login_with_label: LoginWithLabel,
) -> Result<(), String> {
    if let LoginWithAccount {
        account: Some(account),
        password: Some(password),
        save,
        default,
    } = login_with_account
    {
        let user = User::new(account, password);
        do_login(&user, true);

        if let Some(label) = save {
            data.users.insert(label.clone(), user);
            if default {
                data.default = label;
            }
        };
        Ok(())
    } else if let LoginWithLabel { load: Some(label) } = login_with_label {
        data.users
            .contains_key(&label)
            .then(|| {
                do_login(&data.users[&label], true);
                Ok(())
            })
            .unwrap_or_else(|| Err("无法找到该标签的登入信息.".to_owned()))
    } else {
        data.default
            .is_empty()
            .then(|| Err("未设置默认登入信息.".to_owned()))
            .unwrap_or_else(|| {
                do_login(&data.users[&data.default], true);
                Ok(())
            })
    }
}

pub fn do_login(user: &User, with_rich_info: bool) {
    match network::login::login(&user.account, &user.password) {
        Ok(response) => {
            println!("登入成功!\n");
            if with_rich_info {
                let user = ctbox::network::query::query_user_info(None);
                let device = ctbox::network::query::query_user_info(None);
                println!(
                    "您好 {}\n本机ip: {}\n余额: {}\n设备数量: {}",
                    response.uid,
                    response.v46ip,
                    user.map_or_else(
                        |_| "unknow".to_string(),
                        |user| user[0].user_money.to_string()
                    ),
                    device.map_or_else(|_| "unknow".to_string(), |device| device.len().to_string())
                )
            }
        }
        Err(e) => println!("登入失败!\n错误信息: {:?}", e),
    }
}

pub fn logout(_data: &data::Network, _config: &config::Network) -> Result<(), String> {
    do_logout();
    Ok(())
}

pub fn do_logout() {
    match network::logout::logout() {
        Ok(_) => println!("登出成功."),
        Err(e) => println!("登出失败!\n错误信息: {:?}", e),
    }
}

pub fn query(
    _data: &data::Network,
    _config: &config::Network,
    account: &Option<String>,
) -> Result<(), String> {
    do_query_user(account.as_deref());
    println!();
    do_query_device(account.as_deref());
    Ok(())
}

pub fn do_query_user(account: Option<&str>) {
    match network::query::query_user_info(account) {
        Ok(users) => {
            println!(
                "{:<21}{:<11}{:<11}{:<9}",
                "已用流量", "已用时长", "用户余额", "无感知MAC"
            );
            users.iter().for_each(|user| {
                println!(
                    "{:<25}{:<15}{:<14}{:<12}",
                    format!("{}MB", user.user_flow),
                    format!("{}Min", user.user_time),
                    format!("{}元", user.user_money),
                    format!("{}", user.mac.as_ref().unwrap_or(&"无".to_string()))
                )
            });
        }
        Err(e) => println!("用户信息获取失败。错误信息: {:?}", e),
    }
}

pub fn do_query_device(account: Option<&str>) {
    match network::query::query_device_info(account) {
        Ok(devices) => {
            println!(
                "{:<21}{:<10}{:<13}{:<10}",
                "登录时间", "认证服务器", "设备IP", "设备MAC"
            );

            devices.iter().for_each(|device| {
                println!(
                    "{:<25}{:<15}{:<15}{:<12}",
                    format!("{}", device.login_time),
                    format!("{}", device.bas_id),
                    format!("{}", device.login_ip),
                    format!("{}", device.mac_address)
                )
            });
        }
        Err(e) => println!("设备信息获取失败。错误信息: {:?}", e),
    }
}

pub fn encrypt(
    _data: &data::Network,
    _config: &config::Network,
    decrypt: bool,
    source: &str,
) -> Result<(), String> {
    do_encrypt(decrypt, source);
    Ok(())
}

pub fn do_encrypt(decrypt: bool, source: &str) {
    println!("{}", network::encrypt::encrypt(decrypt, source));
}

#[derive(Default, Debug)]
pub enum Level {
    #[default]
    Wa,
    Erro,
    Fatal,
}

#[derive(Default)]
pub struct ErroHandle {
    pub level: Level,
    pub message: String,
}

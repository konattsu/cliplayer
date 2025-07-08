/// 主にコマンドライン引数からvideo id読み取るための型
#[derive(Debug, Clone)]
pub struct VideoIds(Vec<crate::model::VideoId>);

impl VideoIds {
    /// - Error: ids.is_empty()
    fn new(ids: Vec<crate::model::VideoId>) -> Result<Self, &'static str> {
        if ids.is_empty() {
            return Err("VideoIds cannot be empty");
        }
        Ok(VideoIds(ids))
    }

    pub fn into_ids(self) -> Vec<crate::model::VideoId> {
        self.0
    }

    pub fn as_ids(&self) -> &[crate::model::VideoId] {
        &self.0
    }
}

impl std::fmt::Display for VideoIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ids_str = self
            .0
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", ids_str)
    }
}

impl std::str::FromStr for VideoIds {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ids: Vec<crate::model::VideoId> = s
            .split(|c| ",; \t\n\r".contains(c))
            .filter(|id| !id.is_empty())
            .map(|id| crate::model::VideoId::new(id.trim().to_string()))
            .collect::<Result<Vec<_>, _>>()?;
        VideoIds::new(ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_ids_from_str() {
        let input = "11111111111, 22222222222;;\n\n \r \r \r \r \t33333333333";
        let video_ids: VideoIds = input.parse().unwrap();
        assert_eq!(video_ids.clone().into_ids().len(), 3);
        assert_eq!(
            video_ids.to_string(),
            "11111111111, 22222222222, 33333333333"
        );
    }
}

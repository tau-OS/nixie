pub mod timer {
    use he::EmptyPage;

    pub fn build() -> EmptyPage {
        let page = EmptyPage::builder()
            .title("Empty lol")
            .build();

        return page;
    }
}

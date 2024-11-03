use gabrielkaszewski_rs::app::App;
use loco_rs::testing;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_get_data() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/data/").await;
        assert_eq!(res.status_code(), 200);

        // you can assert content like this:
        // assert_eq!(res.text(), "content");
    })
    .await;
}


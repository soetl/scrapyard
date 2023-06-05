use s3::{
    creds::Credentials, error::S3Error, request::ResponseData, Bucket, BucketConfiguration, Region,
};

pub struct FileManager {
    bucket: Bucket,
}

impl FileManager {
    pub async fn new() -> Self {
        let bucket_name = "scrapyard";
        let region = Region::Custom {
            region: "eu-central-1".to_owned(),
            endpoint: "http://storage:9000".to_owned(),
        };
        let credentials = Credentials::default().unwrap();

        let mut bucket = Bucket::new(bucket_name, region.to_owned(), credentials.to_owned())
            .unwrap()
            .with_path_style();

        if let Err(e) = bucket.put_object("ping", "pong".as_bytes()).await {
            if let S3Error::Http(404, _) = e {
                let create_bucket_response = Bucket::create_with_path_style(
                    bucket_name,
                    region,
                    credentials,
                    BucketConfiguration::default(),
                )
                .await
                .unwrap();

                bucket = create_bucket_response.bucket;

                bucket.put_object("ping", "pong".as_bytes()).await.unwrap();
            } else {
                panic!("Error: {:?}", e);
            }
        }

        Self { bucket }
    }

    pub async fn upload(&self, filename: &str, data: &[u8]) -> Result<ResponseData, S3Error> {
        self.bucket.put_object(filename, data).await
    }

    pub async fn download(&self, filename: &str) -> Result<ResponseData, S3Error> {
        self.bucket.get_object(filename).await
    }

    pub async fn delete(&self, filename: &str) -> Result<ResponseData, S3Error> {
        self.bucket.delete_object(filename).await
    }
}

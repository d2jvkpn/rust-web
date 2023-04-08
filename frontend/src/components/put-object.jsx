import OSS from "ali-oss/dist/aliyun-oss-sdk.js";
import { S3Client, PutObjectCommand } from "@aws-sdk/client-s3";
import { message } from "antd";

import { getSts } from "js/api.js";

function aliyunOssPut(request, sts, callback) {
  let client = new OSS({
    accessKeyId: sts.accessKeyId,
    accessKeySecret: sts.accessKeySecret,
    region: "oss-" + sts.region,
    bucket: sts.bucket,
    secure: true,
    stsToken: sts.securityToken,
  });

  client.put(request.key, request.target).then(res => {
    // console.log(`~~ aliyunOssPut: ${JSON.stringify(res)}`);
    if (callback) {
      callback(res.url);
    }
  }).catch(err => {
    message.error("failed to upload"]);
    console.log(`!!! aliyunOssPut: ${err}`);
  });
}

function awsS3Put(request, sts, callback) {
  let client = new S3Client({
    region: sts.region,
    credentials: {
      accessKeyId: sts.accessKeyId,
      secretAccessKey: sts.secretAccessKey,
      sessionToken: sts.sessionToken,
    },
  });

  let command = new PutObjectCommand({
    Bucket: sts.bucket,
    Key: request.key,
    Body: request.target,
  });

  client.send(command).then(res => {
    // console.log(`~~ awsS3Put: ${JSON.stringify(res)}`);
    if (callback) {
      let link = `https://${sts.bucket}.s3.${sts.region}.amazonaws.com/${request.key}`;
      if (callback) {
        callback(link);
      }
    }
  }).catch(err => {
    message.error("failed to upload");
    console.log(`!!! awsS3Put: ${err}`);
  });
}

export putObject = (kind, key, target, callback) => {
  getSts({kind: kind, key: key}, res => {
    if (res.code !== 0) {
      message.error("failed to get sts");
      return;
    }

    let request = { kind, key, target };
    let { provider, sts } = res.data;

    if (provider === "aliyun_oss") {
      aliyunOssPut(request, sts, callback);
    } else if (provider === "aws_s3") {
      awsS3Put(request, sts, callback);
    } else {
      message.error(`unknown sts provider: ${provider}`);
    }
  });
}

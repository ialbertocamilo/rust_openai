const AWS = require('aws-sdk');
const sns = new AWS.SNS();

exports.handler = async (event) => {
    const message = event.Records[0].Sns.Message;
    console.log('Received message:', message);

    // Your code logic here

    return {
        statusCode: 200,
        body: 'Lambda function executed successfully'
    };
};
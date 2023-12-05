
const axios = require('axios');

async function generateImage(prompt) {
    const response = await axios.post('https://api.eleuther.ai/completion', {
        'context': prompt,
        'model': 'davinci',
        'token_max_length': 512,
        'temperature': 0.6,
        'top_p': 1.0
    });

    return response.data;
}

generateImage('A cat sitting on a mat').then(data => console.log(data));

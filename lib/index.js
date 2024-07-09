const genai = require('../index.node');

class GeminiFlash {
    constructor(api_key) {
        this.api_key = api_key;
    }

    async generate_content(prompt) {
        try {
            const response = await genai.generate_content(this.api_key, prompt);
            return response;
        } catch (error) {
            console.error("Error generating content:", error);
            throw error;
        }
    }

    async image_search(prompt, base64img) {
        try {
            const response = await genai.image_search(this.api_key, prompt, base64img);
            return response;
        } catch (error) {
            console.error("Error generating content:", error);
            throw error;
        }
    }
}

module.exports = GeminiFlash;

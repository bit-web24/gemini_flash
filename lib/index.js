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
}

module.exports = GeminiFlash;

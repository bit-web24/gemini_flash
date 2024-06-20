

# GeminiFlash

GeminiFlash is a Node.js library that leverages a Rust backend for generating content using Google's Gemini-1.5-Flash Generative AI Model. This library is designed to be efficient and easy to use, thanks to the power of Rust and the simplicity of JavaScript.

## Installation

To install the GeminiFlash library, you can use npm:

```bash
npm install gemini_flash
```

## Usage

Here's a basic example of how to use the GeminiFlash library:

```javascript
const GeminiFlash = require('gemini_flash');

const gemini_flash = new GeminiFlash('api-key-here');

(async () => {
    try {
        const prompt = "what is a computer?";
        const content = await gemini_flash.generate_content(prompt);
        console.log("Generated Content:", content.text);
    } catch (error) {
        console.error("Error:", error);
    }
})();
```

### API

#### `GeminiFlash`

##### `constructor(api_key)`

Creates a new instance of the `GeminiFlash` class.

- `api_key`: Your API key for the Gemini API.

##### `generate_content(prompt)`

Generates content based on the given prompt.

- `prompt`: The prompt string to generate content for.

**Returns**: A Promise that resolves to an object containing the generated content.

### Example

```javascript
const GeminiFlash = require('gemini_flash');

const gemini_flash = new GeminiFlash('your-api-key');

(async () => {
    try {
        const prompt = "Explain the concept of artificial intelligence.";
        const content = await gemini_flash.generate_content(prompt);
        console.log("Generated Content:", content.text);
    } catch (error) {
        console.error("Error:", error);
    }
})();
```

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) (v18.20.3 or later)
- [npm](https://www.npmjs.com/) (v10.7.0 or later)
- [Rust](https://www.rust-lang.org/)
- [Neon](https://neon-rs.dev/)

### Building the Project

To build the project, run:

```bash
npm run build
```

### Running Tests

To run the tests, use:

```bash
npm test
```

### Additional Build Scripts

- **Debug build**: `npm run debug`
- **Release build**: `npm run build`
- **Cross build**: `npm run cross`

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you find a bug or have a feature request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
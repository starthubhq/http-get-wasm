#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

class HttpGetRunner {
  constructor() {
    this.wasmPath = path.join(__dirname, 'target/wasm32-wasip1/release/http-get-wasm.wasm');
  }

  async run(input) {
    try {
      // Check if WASM file exists
      if (!fs.existsSync(this.wasmPath)) {
        throw new Error(`WASM file not found at ${this.wasmPath}. Please run 'cargo +nightly component build --release' first.`);
      }

      // Check if wasmtime is available
      const wasmtimeAvailable = await this.checkWasmtime();
      if (!wasmtimeAvailable) {
        throw new Error('wasmtime is not available. Please install it with: brew install wasmtime');
      }

      // Run the WASM module using wasmtime
      const result = await this.runWasmWithWasmtime(input);
      return result;

    } catch (error) {
      return {
        success: false,
        error: error.message,
        logs: `Error: ${error.message}`
      };
    }
  }

  async checkWasmtime() {
    return new Promise((resolve) => {
      const wasmtime = spawn('wasmtime', ['--version']);
      wasmtime.on('close', (code) => {
        resolve(code === 0);
      });
      wasmtime.on('error', () => {
        resolve(false);
      });
    });
  }

  async runWasmWithWasmtime(input) {
    return new Promise((resolve, reject) => {
      const inputJson = JSON.stringify(input);
      
      // Spawn wasmtime process
      const wasmtime = spawn('wasmtime', [
        '-S', 'http',  // Enable HTTP support
        this.wasmPath
      ], {
        stdio: ['pipe', 'pipe', 'pipe']
      });

      let stdout = '';
      let stderr = '';

      wasmtime.stdout.on('data', (data) => {
        stdout += data.toString();
      });

      wasmtime.stderr.on('data', (data) => {
        stderr += data.toString();
      });

      wasmtime.on('close', (code) => {
        if (code === 0) {
          // Parse the starthub output format
          const starthubMatch = stdout.match(/::starthub:state::(.+)/);
          
          if (starthubMatch) {
            try {
              const result = JSON.parse(starthubMatch[1]);
              resolve({
                success: true,
                data: result,
                logs: stderr.trim()
              });
            } catch (e) {
              reject(new Error(`Failed to parse starthub output: ${e.message}`));
            }
          } else {
            reject(new Error('No valid starthub output found'));
          }
        } else {
          reject(new Error(`wasmtime exited with code ${code}: ${stderr}`));
        }
      });

      wasmtime.on('error', (error) => {
        reject(new Error(`Failed to start wasmtime: ${error.message}`));
      });

      // Send input to wasmtime
      wasmtime.stdin.write(inputJson);
      wasmtime.stdin.end();
    });
  }
}

// CLI usage
async function main() {
  const args = process.argv.slice(2);
  
  if (args.length === 0) {
    console.log('Usage: node index.js <input-json>');
    console.log('Example: node index.js \'{"params":{"url":"https://httpbin.org/get"}}\'');
    process.exit(1);
  }

  try {
    const input = JSON.parse(args[0]);
    const runner = new HttpGetRunner();
    const result = await runner.run(input);
    
    if (result.success) {
      console.log(JSON.stringify(result.data, null, 2));
      if (result.logs) {
        console.error(result.logs);
      }
    } else {
      console.error('Error:', result.error);
      if (result.logs) {
        console.error('Logs:', result.logs);
      }
      process.exit(1);
    }
  } catch (error) {
    console.error('Error:', error.message);
    process.exit(1);
  }
}

// Export for programmatic usage
module.exports = { HttpGetRunner };

// Run CLI if this file is executed directly
if (require.main === module) {
  main();
}

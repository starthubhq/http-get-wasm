#!/usr/bin/env node

const { HttpGetRunner } = require('./index.js');

async function testHttpGetWasm() {
  console.log('Testing http-get-wasm with Node.js...\n');
  
  const runner = new HttpGetRunner();
  
  // Test 1: Basic GET request
  console.log('Test 1: Basic GET request to httpbin.org');
  try {
    const result1 = await runner.run({
      params: {
        url: "https://httpbin.org/get"
      }
    });
    
    if (result1.success) {
      console.log('✅ Success!');
      console.log('Status:', result1.data.status);
      console.log('Response body preview:', result1.data.body.substring(0, 200) + '...');
    } else {
      console.log('❌ Failed:', result1.error);
    }
  } catch (error) {
    console.log('❌ Error:', error.message);
  }
  
  console.log('\n' + '='.repeat(50) + '\n');
  
  // Test 2: GET request with headers
  console.log('Test 2: GET request with custom headers');
  try {
    const result2 = await runner.run({
      params: {
        url: "https://httpbin.org/headers",
        headers: {
          "User-Agent": "Node.js-WASM-Test/1.0",
          "Accept": "application/json"
        }
      }
    });
    
    if (result2.success) {
      console.log('✅ Success!');
      console.log('Status:', result2.data.status);
      console.log('Response body preview:', result2.data.body.substring(0, 200) + '...');
    } else {
      console.log('❌ Failed:', result2.error);
    }
  } catch (error) {
    console.log('❌ Error:', error.message);
  }
  
  console.log('\n' + '='.repeat(50) + '\n');
  
  // Test 3: Invalid URL (should handle gracefully)
  console.log('Test 3: Invalid URL (error handling)');
  try {
    const result3 = await runner.run({
      params: {
        url: "https://invalid-domain-that-does-not-exist.com"
      }
    });
    
    if (result3.success) {
      console.log('✅ Request completed (unexpected)');
      console.log('Status:', result3.data.status);
    } else {
      console.log('✅ Error handled gracefully:', result3.error);
    }
  } catch (error) {
    console.log('✅ Error caught:', error.message);
  }
}

// Run tests
testHttpGetWasm().catch(console.error);

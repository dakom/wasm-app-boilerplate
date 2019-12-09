const { resolve } = require('path');
module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  globals: {
    'ts-jest': {
      tsConfig: 'tsconfig.json'
    }
  },
  moduleNameMapper: {
    '^@events/(.*)$': resolve(__dirname, './typescript/events/$1'),
    '^@state/(.*)$': resolve(__dirname, './typescript/state/$1'),
    '^@ui/(.*)$': resolve(__dirname, './typescript/ui/$1'),
    '^@utils/(.*)$': resolve(__dirname, './typescript/utils/$1'),
    '^@config/(.*)$': resolve(__dirname, './typescript/config/$1')
  },
};
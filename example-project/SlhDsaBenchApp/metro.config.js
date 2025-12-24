const path = require('path');
const { getDefaultConfig, mergeConfig } = require('@react-native/metro-config');

/**
 * Metro configuration
 * https://reactnative.dev/docs/metro
 *
 * @type {import('@react-native/metro-config').MetroConfig}
 */
const libRoot = path.resolve(__dirname, '../../react-native-slh-dsa');

const config = {
  watchFolders: [libRoot],
  resolver: {
    extraNodeModules: {
      'react-native-slh-dsa': libRoot,
    },
    nodeModulesPaths: [path.resolve(__dirname, 'node_modules')],
  },
};

module.exports = mergeConfig(getDefaultConfig(__dirname), config);

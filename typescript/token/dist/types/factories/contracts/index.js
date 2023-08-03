'use strict';
var __createBinding =
  (this && this.__createBinding) ||
  (Object.create
    ? function (o, m, k, k2) {
        if (k2 === undefined) k2 = k;
        var desc = Object.getOwnPropertyDescriptor(m, k);
        if (
          !desc ||
          ('get' in desc ? !m.__esModule : desc.writable || desc.configurable)
        ) {
          desc = {
            enumerable: true,
            get: function () {
              return m[k];
            },
          };
        }
        Object.defineProperty(o, k2, desc);
      }
    : function (o, m, k, k2) {
        if (k2 === undefined) k2 = k;
        o[k2] = m[k];
      });
var __setModuleDefault =
  (this && this.__setModuleDefault) ||
  (Object.create
    ? function (o, v) {
        Object.defineProperty(o, 'default', { enumerable: true, value: v });
      }
    : function (o, v) {
        o['default'] = v;
      });
var __importStar =
  (this && this.__importStar) ||
  function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null)
      for (var k in mod)
        if (k !== 'default' && Object.prototype.hasOwnProperty.call(mod, k))
          __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
  };
Object.defineProperty(exports, '__esModule', { value: true });
exports.HypNative__factory =
  exports.HypERC721Collateral__factory =
  exports.HypERC721__factory =
  exports.HypERC20Collateral__factory =
  exports.HypERC20__factory =
  exports.test =
  exports.libs =
  exports.extensions =
    void 0;
/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */
exports.extensions = __importStar(require('./extensions'));
exports.libs = __importStar(require('./libs'));
exports.test = __importStar(require('./test'));
var HypERC20__factory_1 = require('./HypERC20__factory');
Object.defineProperty(exports, 'HypERC20__factory', {
  enumerable: true,
  get: function () {
    return HypERC20__factory_1.HypERC20__factory;
  },
});
var HypERC20Collateral__factory_1 = require('./HypERC20Collateral__factory');
Object.defineProperty(exports, 'HypERC20Collateral__factory', {
  enumerable: true,
  get: function () {
    return HypERC20Collateral__factory_1.HypERC20Collateral__factory;
  },
});
var HypERC721__factory_1 = require('./HypERC721__factory');
Object.defineProperty(exports, 'HypERC721__factory', {
  enumerable: true,
  get: function () {
    return HypERC721__factory_1.HypERC721__factory;
  },
});
var HypERC721Collateral__factory_1 = require('./HypERC721Collateral__factory');
Object.defineProperty(exports, 'HypERC721Collateral__factory', {
  enumerable: true,
  get: function () {
    return HypERC721Collateral__factory_1.HypERC721Collateral__factory;
  },
});
var HypNative__factory_1 = require('./HypNative__factory');
Object.defineProperty(exports, 'HypNative__factory', {
  enumerable: true,
  get: function () {
    return HypNative__factory_1.HypNative__factory;
  },
});
//# sourceMappingURL=index.js.map
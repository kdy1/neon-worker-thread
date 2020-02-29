const {
  Worker,
  isMainThread,
  parentPort,
  workerData
} = require("worker_threads");
var addon = require("../native");

if (isMainThread) {
  module.exports = function parseJSAsync(script) {
    return new Promise((resolve, reject) => {
      const worker = new Worker(__filename, {
        workerData: script
      });
      worker.on("message", resolve);
      worker.on("error", reject);
      worker.on("exit", code => {
        if (code !== 0)
          reject(new Error(`Worker stopped with exit code ${code}`));
      });
    });
  };
} else {
  addon.greet();
  addon.greetAsync(function(err, v) {
    console.log("Error: ", err);
    console.log("Result: ", v);
  });
}

const Debug = require('debug');
const Promise = require('bluebird');
const bugsnagClient = require('./bugsnag');

// eslint-disable-next-line import/no-unresolved
const engine = require('@clevy/csml-manager');

const debug = new Debug('CSML:manager:worker');
const { STAGE } = process.env;

/**
 * SNS handler
 *
 * @param {*} event
 */
async function handler(SnsEvent) {
  // λ warmup in cronjobs
  if (!SnsEvent || !SnsEvent.Records) return;

  try {
    await Promise.map(SnsEvent.Records, async record => {
      const message = JSON.parse(record.Sns.Message);
      const { bot, request } = message;

      try {
        await engine.run(request, bot);
      }
      catch (err) {
        console.error('CSML:manager-api:agent', err);
        bugsnagClient.notify(err, { metaData: { custom: message } });
      }

    });
  }
  catch (err) {
    console.error('CSML:manager-api:agent', 'ERR_AGENT_HANDLING', err);
    bugsnagClient.notify(err, { metaData: { custom: SnsEvent.Records } });
  }

}

module.exports.handler = handler;

const Debug = require('debug');
const morgan = require('morgan');
const express = require('express');
const bodyParser = require('body-parser');
const serverless = require('serverless-http');

// eslint-disable-next-line import/no-unresolved
const engine = require('@clevy/csml-manager');

const bugsnagClient = require('./bugsnag');

const app = express();
const bugsnagMiddleware = bugsnagClient.getPlugin('express');

const debug = new Debug('CSML:manager:index');

app.disable('x-powered-by');
app.use(bodyParser.urlencoded({ limit: '50mb', extended: false }));
app.use(bodyParser.json({ limit: '50mb', extended: true }));

const { STAGE } = process.env;

if (STAGE === 'local') app.use(morgan('dev'));
else if (STAGE !== 'test') app.use(morgan('tiny'));

app.use(bugsnagMiddleware.requestHandler);

function OK(req, res) { res.send('Healthy'); }
function favicon(req, res) { res.status(204); }

async function run(req, res) {
  const { bot, event } = req.body;
  if (event.payload && event.payload.content && event.payload.content.close_flows === true) {
    await engine.closeAllConversations(event.client);
  }
  const data = engine.run(event, bot);
  return res.json(data);
}

async function validateFlow(req, res) {
  const { content } = req.body;
  const data = engine.validFlow(content);
  return res.json(data);
}

async function closeAllConversations(req, res) {
  const data = engine.closeAllConversations(req.body);
  return res.json(data);
}

app.get('/', OK);
app.get('/favicon.ico', favicon);
app.post('/run', run);
app.post('/flows/validate', validateFlow);
app.post('/conversations/close', closeAllConversations);

app.use(bugsnagMiddleware.errorHandler);

// `next` parameter must be present
// eslint-disable-next-line no-unused-vars
app.use((err, req, res, next) => {
  debug(err);
  if (res.headersSent) return;
  return res.status(err.status || err.statusCode || 500).send(err);
});

/**
 * HTTP handler
 *
 * @param {*} event
 * @param {*} context
 */
async function handler(event, context) {
  // λ warmup in cronjobs - headers not set by API Gateway
  if (!event || event.source === 'aws.events') return;

  const wrapper = serverless(app);
  return wrapper(event, context);
}

module.exports.handler = handler;

const _get = require('lodash/get');
const bugsnag = require('@bugsnag/js');
const bugsnagExpress = require('@bugsnag/plugin-express');

const { STAGE, GIT_IS_DIRTY, GIT_COMMIT_SHORT, BUGSNAG_API_KEY } = process.env;

const bugsnagClient = bugsnag({
  apiKey: STAGE !== 'local' ? BUGSNAG_API_KEY : 'offline',
  logger: null,
  beforeSend(report) {
    report.context = _get(report, 'metaData.request.path');
    report.appVersion = GIT_COMMIT_SHORT;
    if (GIT_IS_DIRTY) report.appVersion += '-dirty';
    report.releaseStage = STAGE;
  },
});
bugsnagClient.use(bugsnagExpress);

module.exports = bugsnagClient;

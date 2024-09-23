import {
    browserProfilingIntegration,
    browserTracingIntegration,
    handleErrorWithSentry,
    replayIntegration
} from "@sentry/sveltekit";
import * as Sentry from '@sentry/sveltekit';

Sentry.init({
    dsn: 'https://405651d1f519346b182acb1626e78183@o4507958552297472.ingest.us.sentry.io/4507959128358912',

    tracesSampleRate: 1.0,
    tracePropagationTargets: ["localhost"],
    // This sets the sample rate to be 10%. You may want this to be 100% while
    // in development and sample at a lower rate in production
    replaysSessionSampleRate: 1.0,

    // If the entire session is not sampled, use the below sample rate to sample
    // sessions when an error occurs.
    replaysOnErrorSampleRate: 1.0,

    // If you don't want to use Session Replay, just remove the line below:
    integrations: [
        replayIntegration(),
        browserTracingIntegration(),
        browserProfilingIntegration(),
    ],
});

// If you have a custom error handler, pass it to `handleErrorWithSentry`
export const handleError = handleErrorWithSentry();

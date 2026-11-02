import {defineConfig} from '@playwright/test';
const live=process.env.RACKET_BASE_URL;
const site=process.env.RACKET_SITE_DIR || '../../target/racket-release';
export default defineConfig({testDir:'./tests',workers:1,retries:0,timeout:30000,
 use:{baseURL:live || 'http://127.0.0.1:4193/RACKET/workbench/'},
 reporter:[['list'],['json',{outputFile:live?'.cache/live-browser-results.json':'.cache/browser-results.json'}]],
 webServer:live?undefined:{command:`python -m http.server 4193 --directory "${site}"`,url:'http://127.0.0.1:4193/RACKET/workbench/',reuseExistingServer:false}
});

To run or change js:

```
npm run server
```
All changes to js code will be hotloaded while server is running.

To compile rust code changes:
```
1. cargo web build
2. Go to /target/wasm32-unknown-unknown/debug/scroller.js
3. Comment out lines 20 thru 26
4. npm run server
```

Work in progress :)

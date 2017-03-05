# Cowboy

Large refactors made easy.

:construction: **Work In Progress**

## Usage

`cowboy <script> <pattern>`

### Transfrom Script Example

```javascript
module.exports = (path, data) => `
/* 🤠 */
${data}
`;
```

## Benchmarks

```bash
time cowboy transform.js "./src/**/*.js"
367 files refactored!
0.10 real         0.06 user         0.04 sys
```

# Impact Sweeper Terminology Awareness

Identify terms semantically related to user's `concept` input while reading mustCheck specs. Extract only domain terms close to that concept, not every noun in the file; if concept is '流程', extract '工作流', 'workflow', '工作流程' etc. and ignore unrelated terms such as '拓扑排序' or '制品'.

For each extracted term, count occurrences and record the spec names where it appears. Use the spec identifier returned by `openspec list --specs --json` when available; otherwise use the spec directory name without path prefixes or file extensions. Sort extracted terms by descending count, then by term.

Record in `terminologyObservations` field:

```json
{
  "userInput": "string",
  "foundInSpecs": [
    {
      "term": "string",
      "specs": ["string"],
      "count": 1
    }
  ]
}
```

Report facts only, no judgment or recommendations. Do not decide whether terms are correct or should be unified. If terminology extraction fails, omit `terminologyObservations` and keep the report usable with normal impact fields.
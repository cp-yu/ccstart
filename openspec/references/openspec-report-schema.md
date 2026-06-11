# Impact Sweeper JSON Report Schema

```json
{
  "concept": "string",
  "projectRoot": "string",
  "terminologyObservations": {
    "userInput": "string",
    "foundInSpecs": [
      {
        "term": "string",
        "specs": ["string"],
        "count": 1
      }
    ]
  },
  "termMappings": [
    {
      "userTerm": "string",
      "projectTerms": ["string"],
      "evidence": ["string"]
    }
  ],
  "opsx": {
    "nodes": [
      {
        "id": "string",
        "reason": "string"
      }
    ],
    "relationsExpanded": [
      {
        "from": "string",
        "to": "string",
        "type": "string"
      }
    ],
    "coverageGaps": ["string"]
  },
  "mustChange": [
    {
      "target": "string",
      "reason": "string",
      "evidence": ["string"]
    }
  ],
  "mustCheck": [
    {
      "target": "string",
      "reason": "string",
      "evidence": ["string"]
    }
  ],
  "coverageGaps": ["string"],
  "questions": ["string"]
}
```

Field names are canonical. Item values may use natural language. Reports under openspec/sweeper/ are working notes, not proposal, design, tasks, specs, OPSX delta, sync input, or archive input.
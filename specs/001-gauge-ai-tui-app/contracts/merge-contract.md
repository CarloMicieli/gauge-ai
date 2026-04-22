# Contract: Ollama Merge Reconciliation

## Purpose
Defines the input/output contract and validation rules for duplicate-record reconciliation using Ollama.

## Merge Trigger
- Trigger only when a record with the same `(manufacturer, product_code)` already exists AND source fingerprint differs from stored fingerprint.

## Input Payload

```json
{
  "existing_data": {
    "manufacturer": "string",
    "product_code": "string",
    "name": "string",
    "description": "string",
    "details": "string",
    "scale": "string",
    "epoch": "string",
    "railway_company": "string",
    "image_urls": ["string"],
    "specifications": {"key": "value"}
  },
  "new_data": {
    "manufacturer": "string",
    "product_code": "string",
    "name": "string",
    "description": "string",
    "details": "string",
    "scale": "string",
    "epoch": "string",
    "railway_company": "string",
    "image_urls": ["string"],
    "specifications": {"key": "value"}
  },
  "rules": {
    "preserve_identity_fields": ["manufacturer", "product_code"],
    "prefer": "technical_specs_over_marketing",
    "include_unique_specs_from_both_sources": true
  }
}
```

## Output Payload (required)

```json
{
  "manufacturer": "string",
  "product_code": "string",
  "name": "string",
  "description": "string",
  "details": "string",
  "scale": "string",
  "epoch": "string",
  "railway_company": "string",
  "image_urls": ["string"],
  "specifications": {"key": "value"}
}
```

## Validation Rules
- Reject merge if `manufacturer` or `product_code` differ from existing record.
- Reject merge if output is not valid JSON matching expected schema.
- If output invalid after repair attempt, persist raw data with `Unnormalized` status and flag `ManualReview`.

## Persistence Contract
- Successful merge must be committed atomically:
  1. archive prior golden record in `model_versions`.
  2. persist merged current record.
  3. regenerate vector embedding.
  4. write merge audit event.

## Observability Contract
- Every merge attempt emits one audit event with outcome:
  - `Applied`
  - `Rejected`
  - `ManualReview`

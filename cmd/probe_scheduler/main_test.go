package main

import (
	"encoding/json"
	"testing"
)

func TestValidatePort(t *testing.T) {
	tests := []struct {
		name    string
		port    int
		wantErr bool
	}{
		{name: "zero", port: 0, wantErr: true},
		{name: "negative", port: -1, wantErr: true},
		{name: "above maximum", port: 65536, wantErr: true},
		{name: "valid", port: 8742, wantErr: false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := validatePort(tt.port)
			if tt.wantErr && err == nil {
				t.Fatalf("validatePort(%d) returned nil error, want error", tt.port)
			}
			if !tt.wantErr && err != nil {
				t.Fatalf("validatePort(%d) returned error %v, want nil", tt.port, err)
			}
		})
	}
}

func TestSchedulerReportSerializesZeroBridgeResultsAsEmptyList(t *testing.T) {
	results := normalizeSchedulerResults([]MergedResult(nil))
	report := SchedulerReport{
		GeneratedAt:  "2026-06-23T20:08:47Z",
		TotalBridges: len(results),
		Results:      results,
	}

	payload, err := json.Marshal(report)
	if err != nil {
		t.Fatalf("marshal scheduler report: %v", err)
	}

	var decoded map[string]any
	if err := json.Unmarshal(payload, &decoded); err != nil {
		t.Fatalf("unmarshal scheduler report: %v", err)
	}

	if decoded["results"] == nil {
		t.Fatalf("results encoded as null; payload=%s", payload)
	}
	list, ok := decoded["results"].([]any)
	if !ok {
		t.Fatalf("results encoded as %T, want list; payload=%s", decoded["results"], payload)
	}
	if len(list) != 0 {
		t.Fatalf("results length=%d, want 0; payload=%s", len(list), payload)
	}
}

func TestNormalizeSchedulerResultsRejectsNonListValues(t *testing.T) {
	results := normalizeSchedulerResults("not a result list")
	if results == nil {
		t.Fatal("non-list results normalized to nil, want empty list")
	}
	if len(results) != 0 {
		t.Fatalf("non-list results length=%d, want 0", len(results))
	}
}

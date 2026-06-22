package bridge

import (
	"context"
	"testing"
	"time"
)

func TestParse_Vanilla(t *testing.T) {
	line := "1.2.3.4:9999"
	b, err := Parse(line)
	if err != nil {
		t.Fatalf("Parse failed: %v", err)
	}
	if b.Host != "1.2.3.4" || b.Port != 9999 {
		t.Errorf("got host=%s port=%d, want 1.2.3.4:9999", b.Host, b.Port)
	}
	if b.Transport != "vanilla" {
		t.Errorf("got transport=%s, want vanilla", b.Transport)
	}
}

func TestParse_Obfs4(t *testing.T) {
	line := "obfs4 1.2.3.4:9999 cert=test123 iat-mode=0"
	b, err := Parse(line)
	if err != nil {
		t.Fatalf("Parse failed: %v", err)
	}
	if b.Host != "1.2.3.4" || b.Port != 9999 {
		t.Errorf("got host=%s port=%d, want 1.2.3.4:9999", b.Host, b.Port)
	}
	if b.Transport != "obfs4" {
		t.Errorf("got transport=%s, want obfs4", b.Transport)
	}
	if b.Params["cert"] != "test123" {
		t.Errorf("missing or wrong cert parameter")
	}
}

func TestParse_WebTunnel(t *testing.T) {
	line := "webtunnel 1.2.3.4:443 url=https://example.com key=secret"
	b, err := Parse(line)
	if err != nil {
		t.Fatalf("Parse failed: %v", err)
	}
	if b.Host != "example.com" || b.Port != 443 {
		t.Errorf("got host=%s port=%d, want example.com:443", b.Host, b.Port)
	}
	if b.Transport != "webtunnel" {
		t.Errorf("got transport=%s, want webtunnel", b.Transport)
	}
}

func TestParse_IPv6(t *testing.T) {
	line := "[2001:db8::1]:9999"
	b, err := Parse(line)
	if err != nil {
		t.Fatalf("Parse failed: %v", err)
	}
	if b.Host != "2001:db8::1" || b.Port != 9999 {
		t.Errorf("got host=%s port=%d, want 2001:db8::1:9999", b.Host, b.Port)
	}
}

func TestParse_BridgePrefix(t *testing.T) {
	line := "Bridge 1.2.3.4:9999"
	b, err := Parse(line)
	if err != nil {
		t.Fatalf("Parse failed: %v", err)
	}
	if b.Host != "1.2.3.4" || b.Port != 9999 {
		t.Errorf("got host=%s port=%d, want 1.2.3.4:9999", b.Host, b.Port)
	}
}

func TestParse_Empty(t *testing.T) {
	_, err := Parse("")
	if err == nil {
		t.Error("Parse should fail on empty line")
	}
}

func TestParse_NoPort(t *testing.T) {
	_, err := Parse("1.2.3.4")
	if err == nil {
		t.Error("Parse should fail when port is missing")
	}
}

func TestTestWithContext_Snowflake(t *testing.T) {
	b := &Bridge{Transport: "snowflake"}
	ctx, cancel := context.WithTimeout(context.Background(), 1*time.Second)
	defer cancel()
	result := TestWithContext(ctx, b, 5*time.Second)
	if !result {
		t.Error("TestWithContext should return true for snowflake")
	}
}

func TestTestWithContext_InvalidBridge(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 1*time.Second)
	defer cancel()
	result := TestWithContext(ctx, nil, 5*time.Second)
	if result {
		t.Error("TestWithContext should return false for nil bridge")
	}
}

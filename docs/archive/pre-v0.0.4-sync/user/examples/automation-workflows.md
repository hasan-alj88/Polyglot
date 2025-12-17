---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/examples/automation-workflows.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Automation Workflow Examples

**Version:** 0.0.2  
**Working Code:** Complete Implementations
**Last Updated:** 2025-12-02

---

## Example 1: Daily Report Automation

Generates and emails reports every day at 2 AM.

### `DailyReport.pg`

```polyglot
[@] @Local::DailyReports:1.0.0.0
[X]

[#] #DataAnalysis
[<] <total_users:pg.int
[<] <active_users:pg.int
[<] <revenue:pg.float
[<] <errors:pg.int
[X]

[|] |AnalyzeData
[i] .date:pg.dt
[t] |T.Call
[o] .analysis: #DataAnalysis

[W] |W.RT.Rust1.8

[r] |RT.Rust.Run.Function
[<] <function:pg.string << "analytics::analyze_daily_data"
[<] <args:pg.serial << {.date
[>] >output:pg.serial >> .result

[r] .analysis: #DataAnalysis << #DataAnalysis {
[*] <total_users:pg.int << .result.total_users,
[*] <active_users:pg.int << .result.active_users,
[*] <revenue:pg.float << .result.revenue,
[*] <errors:pg.int << .result.errors
[*] 

[o] .analysis: #DataAnalysis
[X]

[|] |GenerateReport
[i] .analysis: #DataAnalysis
[i] .date:pg.dt
[t] |T.Call
[o] .report_html:pg.string
[o] .report_path:pg.path

[W] |W.RT.Python3.14
[<] <requirements:pg.file << \\FileDir\\python\\requirements.txt

[r] |RT.Python.Run.File
[<] <file:pg.path << \\FileDir\\python\\generate_report.py
[<] <input:pg.serial << {.analysis, .date
[>] >output:pg.serial >> .results

[r] .report_html:pg.string << .results.html
[r] .report_path:pg.path << .results.report

[o] .report_html:pg.string
[o] .report_path:pg.path
[X]

[|] |SendEmail
[i] .recipient:pg.string
[i] .subject:pg.string
[i] .body_html:pg.string
[i] .attachment:pg.path
[t] |T.Call
[o] .sent:pg.bool

[W] |W.RT.Python3.14

[r] |RT.Python.Run.File
[<] <file:pg.path << \\FileDir\\python\\send_email.py
[<] <input:pg.serial << {.recipient, .subject, .body_html, .attachment
[>] >output:pg.serial >> .result

[r] .sent:pg.bool << .result.sent

[o] .sent:pg.bool
[X]

[|] |DailyReportWorkflow
[i] !No.Input
[t] |T.DT.Daily
[<] <hour:pg.int << 2
[<] <minute:pg.int << 0
[o] !No.Error
[o] !EmailError

[r] .today:pg.dt << |DT.Today""
[r] .yesterday:pg.dt << |DT.Ago"1d"

[r] |AnalyzeData
[<] <date:pg.dt << .yesterday
[>] >analysis: #DataAnalysis >> .daily_analysis

[r] |GenerateReport
[<] <analysis: #DataAnalysis << .daily_analysis
[<] <date:pg.dt << .yesterday
[>] >report_html:pg.string >> .html_report
[>] >report_path:pg.path >> .report_file

[r] |SendEmail
[<] <recipient:pg.string << "team@company.com"
[<] <subject:pg.string << "Daily Report"
[<] <body_html:pg.string << .html_report
[<] <attachment:pg.path << .report_file
[>] >sent:pg.bool >> .email_sent

[?] .email_sent =? #Boolean.True
[~][o] !No.Error
[~]

[?] *?
[~][o] !EmailError
[~]
[X]
```

---

## Example 2: File Monitoring Workflow

Monitors directory for new files and processes them automatically.

### `FileMonitor.pg`

```polyglot
[@] @Local::FileMonitoring:1.0.0.0
[X]

[|] |ValidateSchema
[i] .file_path:pg.path
[t] |T.Call
[o] .valid:pg.bool
[o] .errors:pg.array{pg\string

[W] |W.RT.Python3.14

[r] |RT.Python.Run.File
[<] <file:pg.path << \\FileDir\\python\\validate_schema.py
[<] <input:pg.serial << {.file_path
[>] >output:pg.serial >> .result

[r] .valid:pg.bool << .result.valid
[r] .errors:pg.array{pg\string << .result.errors

[o] .valid:pg.bool
[o] .errors:pg.array{pg\string
[X]

[|] |ProcessData
[i] .file_path:pg.path
[t] |T.Call
[o] .records_processed:pg.int

[W] |W.RT.Rust1.8

[r] |RT.Rust.Run.Function
[<] <function:pg.string << "processor::process_file"
[<] <args:pg.serial << {.file_path
[>] >output:pg.serial >> .result

[r] .records_processed:pg.int << .result

[o] .records_processed:pg.int
[X]

[|] |FileMonitorWorkflow
[i] .file_path:pg.path
[t] |T.File.Modified
[<] <watch_dir:pg.path << "/data/incoming"
[<] <pattern:pg.string << "*.json"
[o] .count:pg.int
[o] !ValidationError

[r] |ValidateSchema
[<] <file_path:pg.path << .file_path
[>] >valid:pg.bool >> .is_valid
[>] >errors:pg.array{pg\string >> .validation_errors

[?] .is_valid =? #Boolean.True
[~][r] |ProcessData
[~][<] <file_path:pg.path << .file_path
[~][>] >records_processed:pg.int >> .count
[~][o] .count:pg.int
[~]

[?] *?
[~][o] !ValidationError
[~]
[X]
```

---

**Next:** [Multi-Step Pipelines →](multi-step-pipelines.md

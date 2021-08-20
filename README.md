# tonic-connect-repro

Repro for tonic-build bug

```
cameron@chowder ~/s/connect-repro (main)> cargo build
   Compiling connect-repro v0.1.0 (/home/cameron/skamdart/connect-repro)
error[E0592]: duplicate definitions with name `connect`
  --> /home/cameron/skamdart/connect-repro/target/debug/build/connect-repro-3bf3967b9da58014/out/request.rs:15:9
   |
15 | /         pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
16 | |         where
17 | |             D: std::convert::TryInto<tonic::transport::Endpoint>,
18 | |             D::Error: Into<StdError>,
   | |_____________________________________^ duplicate definitions for `connect`
...
65 | /         pub async fn connect(
66 | |             &mut self,
67 | |             request: impl tonic::IntoRequest<super::ConnectRequest>,
68 | |         ) -> Result<tonic::Response<super::ConnectResponse>, tonic::Status> {
   | |___________________________________________________________________________- other definition for `connect`

For more information about this error, try `rustc --explain E0592`.
error: could not compile `connect-repro` due to previous error

```

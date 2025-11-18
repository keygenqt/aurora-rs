Name:       com.keygenqt.aurora_rs
Summary:    Rust and interacts with the main interfaces of Aurora OS.
Version:    0.0.1
Release:    1
License:    BSD 3-Clause

%description
%{summary}

%prep

%build

%install
mkdir -p %{buildroot}/%{_bindir}
mkdir -p %{buildroot}/%{_datadir}/icons
mkdir -p %{buildroot}/%{_datadir}/applications
install -m 0755 ./bin/aurora-rs %{buildroot}/%{_bindir}/%{name}
cp -a icons %{buildroot}/%{_datadir}
cp -a applications %{buildroot}/%{_datadir}

%files
%defattr(-,root,root,-)
%{_bindir}/%{name}
%defattr(644,root,root,-)
%{_datadir}/applications/%{name}.desktop
%{_datadir}/icons/hicolor/*/apps/%{name}.png

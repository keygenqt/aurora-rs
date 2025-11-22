%global __provides_exclude_from ^%{_datadir}/%{name}/lib/.*$
%global __requires_exclude ^lib(freetype)\\.so.*$

Name:       com.keygenqt.aurora_rs
Summary:    Rust and interacts with the main interfaces of Aurora OS.
Version:    0.0.1
Release:    1
License:    BSD 3-Clause

%description
%{summary}

%prep
%autosetup

%build

%install
mkdir -p %{buildroot}/%{_bindir}
mkdir -p %{buildroot}/%{_datadir}/icons
mkdir -p %{buildroot}/%{_datadir}/applications
mkdir -p %{buildroot}/%{_datadir}/%{name}/lib
install -m 0755 bin/aurora-rs %{buildroot}/%{_bindir}/%{name}
install -m 644 libs/%{_target_cpu}/*so* %{buildroot}/%{_datadir}/%{name}/lib
cp -a icons %{buildroot}/%{_datadir}
cp -a applications %{buildroot}/%{_datadir}

%files
%defattr(-,root,root,-)
%{_bindir}/%{name}
%defattr(644,root,root,-)
%{_datadir}/%{name}/lib/*so*
%{_datadir}/applications/%{name}.desktop
%{_datadir}/icons/hicolor/*/apps/%{name}.png

// Route composition only; native data and page responsibilities live in focused modules.
import { HashRouter, Route, Routes } from "react-router-dom";
import { AppShell } from "./app/AppShell";
import { sections } from "./app/navigation";
import { HomePage } from "./app/pages/HomePage";
import { LibraryPage } from "./app/pages/LibraryPage";
import { SettingsPage } from "./app/pages/SettingsPage";
import { ProjectsProvider } from "./features/projects/ProjectsContext";
import "./App.css";

export default function App() {
  return (
    <ProjectsProvider>
      <HashRouter>
        <Routes>
          <Route element={<AppShell />}>
            <Route index element={<HomePage />} />
            {sections.slice(1).map((section) => (
              <Route
                key={section.path}
                path={section.path}
                element={<LibraryPage section={section} />}
              />
            ))}
            <Route path="settings" element={<SettingsPage />} />
            <Route path="*" element={<HomePage />} />
          </Route>
        </Routes>
      </HashRouter>
    </ProjectsProvider>
  );
}

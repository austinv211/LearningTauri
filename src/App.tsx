import { invoke } from "@tauri-apps/api/core";
import React, { useEffect, useState } from "react";
import ProcessCard from "./ProcessCard";
import "bootstrap/dist/css/bootstrap.css";
import "bootstrap/dist/js/bootstrap.js";
import "./App.css";

interface ProcessInfo {
    id: string;
    name: string;
    running_time_formatted: string;
    memory_in_bytes: number;
}

const App: React.FC = () => {
    const [processes, setProcesses] = useState<ProcessInfo[]>([]);

    const [maxMemoryProcess, setMaxMemoryProcess] =
        useState<ProcessInfo | null>(null);
    const [maxRunningProcess, setMaxRunningProcess] =
        useState<ProcessInfo | null>(null);

    useEffect(() => {
        async function fetchData() {
            const processList = await invoke<ProcessInfo[]>("list_process");
            const maxMemory = await invoke<ProcessInfo | null>("max_memory");
            const maxRunning = await invoke<ProcessInfo | null>(
                "max_running_process"
            );

            setProcesses(processList);
            setMaxMemoryProcess(maxMemory);
            setMaxRunningProcess(maxRunning);
        }

        fetchData();
        const interval = setInterval(fetchData, 1000);
        return () => clearInterval(interval);
    }, []);

    return (
        <main className="container">
            {maxMemoryProcess && (
                <ProcessCard
                    title="Max Memory Process"
                    process={maxMemoryProcess}
                />
            )}
            {maxRunningProcess && (
                <ProcessCard
                    title="Max Running Process"
                    process={maxRunningProcess}
                />
            )}

            <table className="table table-dark table-striped table-bordered table-hover">
                <thead>
                    <tr>
                        <th scope="col">ID</th>
                        <th scope="col">Name</th>
                        <th scope="col">Running Time</th>
                        <th scope="col">Memory (Bytes)</th>
                    </tr>
                </thead>
                <tbody>
                    {processes.map((process) => {
                        return (
                            <tr key={process.id}>
                                <td scope="row">{process.id}</td>
                                <td>{process.name}</td>
                                <td>
                                    {process.running_time_formatted}
                                </td>
                                <td>{process.memory_in_bytes} bytes</td>
                            </tr>
                        );
                    })}
                </tbody>
            </table>
        </main>
    );
};

export default App;

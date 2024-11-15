import React from 'react'

interface ProcessCardProps {
   title: string,
   process: {
    id: string,
    name: string,
    running_time_formatted: string,
    memory_in_bytes: number
   };
}


const ProcessCard: React.FC<ProcessCardProps> = ({title, process}) => {
    return (
        <div className="card my-3 bg-dark text-light">
            <div className='card-body'>
                <h5 className='card-title'>{title}</h5>
                <h6 className='card-subtitle mb-2 text-muted'>{process.name} (ID: {process.id})</h6>
                <p className='card-text'>Running Time: {process.running_time_formatted}</p>
                <p className='card-text'>Memory: {process.memory_in_bytes}</p>
            </div>
        </div>
    )

};

export default ProcessCard